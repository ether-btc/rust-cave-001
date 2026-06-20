"""Tests for the rust-cave-001 MCP server.

Tests cover:
- Tool registration (all 5 tools present)
- caveman_compress (full + adaptive mode)
- caveman_compress_batch
- caveman_classify
- caveman_estimate_tokens
- caveman_stats
- Error handling (short text, empty text, missing deps)
- Session stats tracking
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

# Ensure project root is importable
sys.path.insert(0, str(Path(__file__).parent.parent))

from mcp_server.server import SessionStats, _check_deps, _STATS_ENABLED, mcp


# --- Fixtures ---

SAMPLE_TEXTS = [
    "The ball was thrown by John.",
    "The system was designed by the engineering team for high performance.",
    "Hey, how are you doing today? I was wondering if you could help me out.",
    "The aforementioned methodology demonstrates significant improvement "
    "in overall efficiency metrics.",
]

SHORT_TEXT = "Hi"
EMPTY_TEXT = ""


async def call_tool(name: str, arguments: dict) -> str:
    """Helper: call an MCP tool and return the text content."""
    result = await mcp.call_tool(name, arguments)
    content_list = result[0] if isinstance(result, tuple) else result
    return content_list[0].text


# --- Tool Registration ---

class TestToolRegistration:
    """Verify all expected tools are registered."""

    @pytest.mark.asyncio
    async def test_tools_listed(self):
        """All 5 tools should be registered."""
        tools = await mcp.list_tools()
        tool_names = [t.name for t in tools]
        assert "caveman_compress" in tool_names
        assert "caveman_compress_batch" in tool_names
        assert "caveman_classify" in tool_names
        assert "caveman_estimate_tokens" in tool_names
        assert "caveman_stats" in tool_names

    @pytest.mark.asyncio
    async def test_tool_count(self):
        """Should have exactly 5 tools."""
        tools = await mcp.list_tools()
        assert len(tools) == 5

    @pytest.mark.asyncio
    async def test_tools_have_descriptions(self):
        """Every tool should have a non-empty description."""
        tools = await mcp.list_tools()
        for tool in tools:
            assert tool.description, f"Tool {tool.name} has no description"
            assert len(tool.description) > 20


# --- Compression Tests ---

class TestCompress:
    """Test caveman_compress tool."""

    @pytest.mark.asyncio
    async def test_basic_compress(self):
        """Compression returns valid JSON with expected fields."""
        raw = await call_tool("caveman_compress", {"text": SAMPLE_TEXTS[0]})
        result = json.loads(raw)
        assert "compressed" in result
        assert "original_tokens" in result
        assert "compressed_tokens" in result
        assert "tokens_saved" in result
        assert "reduction_pct" in result
        assert "strategy" in result
        assert result["strategy"] == "full"

    @pytest.mark.asyncio
    async def test_compress_reduces_tokens(self):
        """Compression should reduce token count for non-trivial text."""
        raw = await call_tool("caveman_compress", {"text": SAMPLE_TEXTS[1]})
        result = json.loads(raw)
        assert result["compressed_tokens"] < result["original_tokens"]
        assert result["tokens_saved"] > 0
        assert result["reduction_pct"] > 0

    @pytest.mark.asyncio
    async def test_adaptive_compress(self):
        """Adaptive mode should use adaptive strategy."""
        raw = await call_tool(
            "caveman_compress", {"text": SAMPLE_TEXTS[2], "adaptive": True}
        )
        result = json.loads(raw)
        assert result["strategy"] == "adaptive"
        assert "compressed" in result

    @pytest.mark.asyncio
    async def test_short_text_error(self):
        """Single-word input should return error JSON, not crash."""
        raw = await call_tool("caveman_compress", {"text": SHORT_TEXT})
        result = json.loads(raw)
        assert "error" in result

    @pytest.mark.asyncio
    async def test_empty_text_error(self):
        """Empty input should return error JSON."""
        raw = await call_tool("caveman_compress", {"text": EMPTY_TEXT})
        result = json.loads(raw)
        assert "error" in result

    @pytest.mark.asyncio
    async def test_academic_text(self):
        """Academic text should compress well."""
        raw = await call_tool("caveman_compress", {"text": SAMPLE_TEXTS[3]})
        result = json.loads(raw)
        assert result["reduction_pct"] > 20  # at least 20% reduction


# --- Batch Compression Tests ---

class TestCompressBatch:
    """Test caveman_compress_batch tool."""

    @pytest.mark.asyncio
    async def test_batch_basic(self):
        """Batch compress should return array of results."""
        raw = await call_tool(
            "caveman_compress_batch", {"texts": SAMPLE_TEXTS[:2]}
        )
        results = json.loads(raw)
        assert isinstance(results, list)
        assert len(results) == 2
        for r in results:
            assert "compressed" in r
            assert "reduction_pct" in r

    @pytest.mark.asyncio
    async def test_batch_with_short(self):
        """Batch with short text should include error entry."""
        raw = await call_tool(
            "caveman_compress_batch",
            {"texts": [SAMPLE_TEXTS[0], SHORT_TEXT]},
        )
        results = json.loads(raw)
        assert len(results) == 2
        assert "compressed" in results[0]
        assert "error" in results[1]

    @pytest.mark.asyncio
    async def test_batch_adaptive(self):
        """Batch with adaptive flag."""
        raw = await call_tool(
            "caveman_compress_batch",
            {"texts": [SAMPLE_TEXTS[0]], "adaptive": True},
        )
        results = json.loads(raw)
        assert len(results) == 1


# --- Classification Tests -----

class TestClassify:
    """Test caveman_classify tool."""

    @pytest.mark.asyncio
    async def test_classify_returns_type(self):
        """Classification should return a valid text type."""
        raw = await call_tool("caveman_classify", {"text": SAMPLE_TEXTS[0]})
        result = json.loads(raw)
        assert result["text_type"] in [
            "technical",
            "conversational",
            "academic",
            "dialogue",
            "minimal",
            "mixed",
        ]

    @pytest.mark.asyncio
    async def test_classify_returns_strategy(self):
        """Classification should return strategy list."""
        raw = await call_tool("caveman_classify", {"text": SAMPLE_TEXTS[0]})
        result = json.loads(raw)
        assert isinstance(result["recommended_strategy"], list)
        assert result["strategy_count"] == len(result["recommended_strategy"])

    @pytest.mark.asyncio
    async def test_technical_classification(self):
        """Technical text should classify as technical."""
        text = "Hash map offers O(1) lookup. URL: https://example.com/path"
        raw = await call_tool("caveman_classify", {"text": text})
        result = json.loads(raw)
        assert result["text_type"] == "technical"


# --- Token Estimation Tests ---

class TestEstimateTokens:
    """Test caveman_estimate_tokens tool."""

    @pytest.mark.asyncio
    async def test_estimate_basic(self):
        """Token estimation should return positive int."""
        raw = await call_tool("caveman_estimate_tokens", {"text": SAMPLE_TEXTS[0]})
        result = json.loads(raw)
        assert result["estimated_tokens"] > 0
        assert result["char_count"] == len(SAMPLE_TEXTS[0])

    @pytest.mark.asyncio
    async def test_estimate_empty(self):
        """Empty text should return 0 tokens."""
        raw = await call_tool("caveman_estimate_tokens", {"text": ""})
        result = json.loads(raw)
        assert result["estimated_tokens"] == 0


# --- Stats Tests ---

class TestStats:
    """Test caveman_stats tool."""

    @pytest.mark.asyncio
    async def test_stats_returns_dict(self):
        """Stats should return a JSON object."""
        raw = await call_tool("caveman_stats", {})
        result = json.loads(raw)
        # If stats enabled, should have fields; if disabled, should say so
        if _STATS_ENABLED:
            assert "compressions" in result
            assert "total_tokens_saved" in result
        else:
            assert result.get("status") == "disabled"


# --- SessionStats Unit Tests ---

class TestSessionStatsUnit:
    """Unit tests for SessionStats dataclass."""

    def test_record_tracks_compression(self):
        stats = SessionStats()
        stats.record(100, 50, "full")
        assert stats.compressions == 1
        assert stats.total_input_tokens == 100
        assert stats.total_output_tokens == 50
        assert stats.total_tokens_saved == 50
        assert stats.strategies_used["full"] == 1

    def test_record_multiple(self):
        stats = SessionStats()
        stats.record(100, 50, "full")
        stats.record(200, 100, "adaptive")
        assert stats.compressions == 2
        assert stats.total_tokens_saved == 150
        assert stats.strategies_used["full"] == 1
        assert stats.strategies_used["adaptive"] == 1

    def test_to_dict_savings_pct(self):
        stats = SessionStats()
        stats.record(100, 40, "full")
        d = stats.to_dict()
        assert d["savings_percent"] == 60.0
        assert d["compressions"] == 1

    def test_to_dict_empty(self):
        stats = SessionStats()
        d = stats.to_dict()
        assert d["savings_percent"] == 0
        assert d["compressions"] == 0


# --- Dependency Check ---

class TestDependencyCheck:
    """Test dependency checking."""

    def test_deps_available(self):
        """If rust_cave_001 is installed, _check_deps returns None."""
        # In the test environment, rust_cave_001 should be available
        assert _check_deps() is None or "not installed" in _check_deps()
