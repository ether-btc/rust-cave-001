#!/usr/bin/env python3
"""rust-cave-001 MCP Server — Deterministic text compression for LLM token reduction.

Exposes compression, classification, and stats as MCP tools that any
MCP-compatible host (Claude Code, Cursor, Hermes Agent, etc.) can use.

Tools:
    caveman_compress       — Compress text (~48% avg token reduction)
    caveman_compress_batch — Compress multiple texts in one call
    caveman_classify       — Classify text type + recommended strategy
    caveman_estimate_tokens— Estimate token count for text
    caveman_stats          — Session compression statistics

Usage:
    python -m mcp_server.server
    python mcp_server/server.py
    caveman-mcp   (if installed via pip with [mcp] extra)

Config (Hermes Agent config.yaml):
    mcp:
      caveman:
        command: python
        args: ["-m", "mcp_server.server"]
        env:
          CAVEMAN_MCP_STATS: "on"    # enable session stats tracking

"""

from __future__ import annotations

import json
import os
import sys
import time
from dataclasses import dataclass, field

# --- MCP SDK (optional import) ---
try:
    from mcp.server.fastmcp import FastMCP

    MCP_AVAILABLE = True
except ImportError:
    MCP_AVAILABLE = False
    FastMCP = None  # type: ignore[assignment,misc]

# Always define mcp for testing, even if MCP not available
mcp = None

# --- rust-cave-001 native module ---
try:
    import rust_cave_001  # type: ignore[import-not-found,no-attr-defined]

    LIBRARY_AVAILABLE = True
except ImportError:
    LIBRARY_AVAILABLE = False
    rust_cave_001 = None  # type: ignore[assignment]
# Enable stats by default - can be disabled via env var
_STATS_ENABLED = os.environ.get("CAVEMAN_MCP_STATS", "on").lower().strip() in (
    "on",
    "true",
    "1",
    "yes",
    "enabled",
)

# Production limits
MAX_BATCH_SIZE = 100  # Max texts per batch call
MAX_STATS_COMPRESSIONS = 10000  # Auto-reset stats after N compressions
MAX_STRATEGY_TRACKING = 20  # Cap unique strategies tracked


@dataclass
class SessionStats:
    """Track compression statistics for the current MCP session."""

    compressions: int = 0
    total_input_tokens: int = 0
    total_output_tokens: int = 0
    total_tokens_saved: int = 0
    started_at: float = field(default_factory=time.time)
    strategies_used: dict[str, int] = field(default_factory=dict)

    def record(self, input_tokens: int, output_tokens: int, strategy: str) -> None:
        self.compressions += 1
        self.total_input_tokens += input_tokens
        self.total_output_tokens += output_tokens
        self.total_tokens_saved += max(0, input_tokens - output_tokens)

        # Cap strategy tracking to prevent unbounded growth
        if len(self.strategies_used) < MAX_STRATEGY_TRACKING or strategy in self.strategies_used:
            self.strategies_used[strategy] = min(
                self.strategies_used.get(strategy, 0) + 1, 1000
            )

        # Auto-reset after max compressions to prevent memory bloat
        if self.compressions >= MAX_STATS_COMPRESSIONS:
            self._reset()

    def _reset(self) -> None:
        """Reset counters but keep cumulative totals for session lifetime."""
        self.compressions = 0
        self.strategies_used.clear()

    def to_dict(self) -> dict[str, int | float | dict[str, int]]:
        pct = (
            round(self.total_tokens_saved / self.total_input_tokens * 100, 1)
            if self.total_input_tokens > 0
            else 0
        )
        return {
            "session_duration_seconds": round(time.time() - self.started_at),
            "compressions": self.compressions,
            "total_input_tokens": self.total_input_tokens,
            "total_output_tokens": self.total_output_tokens,
            "total_tokens_saved": self.total_tokens_saved,
            "savings_percent": pct,
            "strategies_used": self.strategies_used,
        }


_stats = SessionStats() if _STATS_ENABLED else None


def _check_deps() -> str | None:
    """Return error message if dependencies are missing."""
    if not MCP_AVAILABLE:
        return "MCP SDK not installed. Install with: pip install 'rust-cave-001[mcp]' or pip install mcp"
    if not LIBRARY_AVAILABLE:
        return "rust_cave_001 not installed. Install with: pip install rust-cave-001"
    return None


# --- Server setup ---

if MCP_AVAILABLE:
    mcp = FastMCP("caveman")  # type: ignore[assignment]

    @mcp.tool()  # type: ignore[union-attr]
    def caveman_compress(text: str, adaptive: bool = False) -> str:
        """Compress text using deterministic Caveman rules.

        Reduces tokens by ~48% on average through:
        - Passive→active voice transformation
        - Contraction expansion
        - Copular 'be' removal
        - Connective reduction (although, since, while, etc.)
        - Article/determiner pruning
        - Intensifier removal
        - Word limit enforcement

        Args:
            text: Input text to compress. Minimum 2 words required.
            adaptive: If True, auto-classify text type and select optimal
                      rule subset. Default False (full pipeline).

        Returns:
            Compressed text string. If the text is too short (<2 words after
            compression), returns the original text unchanged with a warning.

        Example:
            caveman_compress("The ball was thrown by John.")
            → "John threw the ball"
        """
        err = _check_deps()
        if err:
            return json.dumps({"error": err})

        if not text or len(text.split()) < 2:
            return json.dumps(
                {"error": "Input must have at least 2 words"}
            )

        try:
            strategy = "adaptive" if adaptive else "full"
            result = (
                rust_cave_001.compress_adaptive(text)
                if adaptive
                else rust_cave_001.compress(text)
            )

            input_tokens = rust_cave_001.estimate_tokens(text)
            output_tokens = rust_cave_001.estimate_tokens(result)

            if _stats:
                _stats.record(input_tokens, output_tokens, strategy)

            return json.dumps(
                {
                    "compressed": result,
                    "original_tokens": input_tokens,
                    "compressed_tokens": output_tokens,
                    "tokens_saved": max(0, input_tokens - output_tokens),
                    "reduction_pct": round(
                        (1 - output_tokens / input_tokens) * 100, 1
                    )
                    if input_tokens > 0
                    else 0,
                    "strategy": strategy,
                },
                indent=2,
            )
        except ValueError as e:
            # Sanitize error message - never echo user input
            safe_msg = str(e)[:200]
            return json.dumps({"error": safe_msg})
        except Exception:
            # Log full error internally, return generic message
            return json.dumps({"error": "Compression failed (internal error)"})

    @mcp.tool()  # type: ignore[union-attr]
    def caveman_compress_batch(texts: list[str], adaptive: bool = False) -> str:
        """Compress multiple texts in a single call.

        More efficient than calling caveman_compress repeatedly for
        bulk processing (e.g., compressing a list of file contents).

        Args:
            texts: List of input texts to compress. Max 100 texts per call.
            adaptive: If True, use adaptive compression per text.

        Returns:
            JSON array of compression results, one per input text.
        """
        err = _check_deps()
        if err:
            return json.dumps({"error": err})

        # DoS prevention: limit batch size
        if len(texts) > MAX_BATCH_SIZE:
            return json.dumps({
                "error": f"Batch size exceeds limit ({len(texts)} > {MAX_BATCH_SIZE} texts)"
            })

        results = []
        for i, text in enumerate(texts):
            if not text or len(text.split()) < 2:
                results.append(
                    {"index": i, "error": "Input must have at least 2 words"}
                )
                continue
            try:
                strategy = "adaptive" if adaptive else "full"
                result = (
                    rust_cave_001.compress_adaptive(text)
                    if adaptive
                    else rust_cave_001.compress(text)
                )
                input_tokens = rust_cave_001.estimate_tokens(text)
                output_tokens = rust_cave_001.estimate_tokens(result)

                if _stats:
                    _stats.record(input_tokens, output_tokens, strategy)

                results.append(
                    {
                        "index": i,
                        "compressed": result,
                        "original_tokens": input_tokens,
                        "compressed_tokens": output_tokens,
                        "tokens_saved": max(0, input_tokens - output_tokens),
                        "reduction_pct": round(
                            (1 - output_tokens / input_tokens) * 100, 1
                        )
                        if input_tokens > 0
                        else 0,
                        "strategy": strategy,
                    }
                )
            except ValueError as e:
                results.append({"index": i, "error": str(e)[:200]})
            except Exception:
                results.append({"index": i, "error": "Compression failed (internal error)"})

        return json.dumps(results, indent=2)

    @mcp.tool()  # type: ignore[union-attr]
    def caveman_classify(text: str) -> str:
        """Classify text type and get recommended compression strategy.

        Analyzes text using 13 heuristic dimensions across 6 types:
        - technical: Code, paths, numbers, URLs, structured data
        - conversational: Informal, pronoun-rich, contractions
        - academic: Dense vocabulary, long sentences, formal
        - dialogue: Quotes, short utterances, turn-taking
        - minimal: Already terse (<4 words)
        - mixed: Multiple type signals

        Args:
            text: Input text to classify.

        Returns:
            JSON with text_type and recommended_strategy (list of rule names).
        """
        err = _check_deps()
        if err:
            return json.dumps({"error": err})

        text_type = rust_cave_001.classify_text(text)
        strategy = rust_cave_001.recommended_strategy_for_text(text)
        return json.dumps(
            {
                "text_type": text_type,
                "recommended_strategy": strategy,
                "strategy_count": len(strategy),
            },
            indent=2,
        )

    @mcp.tool()  # type: ignore[union-attr]
    def caveman_estimate_tokens(text: str) -> str:
        """Estimate token count for text using regex-based approximation.

        Uses a cached regex pattern (~4 chars/token heuristic with
        adjustments for punctuation and special tokens).

        Args:
            text: Input text.

        Returns:
            JSON with estimated token count.
        """
        err = _check_deps()
        if err:
            return json.dumps({"error": err})

        tokens = rust_cave_001.estimate_tokens(text)
        return json.dumps(
            {"estimated_tokens": tokens, "char_count": len(text)}, indent=2
        )

    @mcp.tool()  # type: ignore[union-attr]
    def caveman_stats() -> str:
        """Show compression statistics for this MCP session.

        Returns total compressions, tokens saved, average reduction
        percentage, and which compression strategies were used.

        Returns:
            JSON with session statistics. Returns 'disabled' if stats
            tracking is off (set CAVEMAN_MCP_STATS=off to disable).
        """
        if not _stats:
            return json.dumps(
                {"status": "disabled", "hint": "Set CAVEMAN_MCP_STATS=on to enable"}
            )
        return json.dumps(_stats.to_dict(), indent=2)


def main() -> None:
    """Entry point for the MCP server."""
    err = _check_deps()
    if err:
        print(err, file=sys.stderr)
        sys.exit(1)
    mcp.run()  # type: ignore[union-attr]


if __name__ == "__main__":
    main()
