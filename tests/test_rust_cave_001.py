#!/usr/bin/env python3
"""Pytest test suite for rust-cave-001.

Run with: cd /srv/sync/projects/rust-cave-001 && source .venv/bin/activate && pytest tests/ -v
"""

import sys
import os
import pytest

# Ensure .venv python is used and library is importable
sys.path.insert(0, os.path.dirname(__file__))
os.environ["LD_LIBRARY_PATH"] = os.path.join(os.path.dirname(__file__), "target", "release") + ":" + os.environ.get("LD_LIBRARY_PATH", "")

from rust_cave_001 import (
    my_compress,
    decompress,
    estimate_tokens,
    get_stats,
    serialize_compressed,
    deserialize_compressed,
    preprocess_text,
)


# =============================================================================
# Compression / Decompression
# =============================================================================

class TestCompression:
    def test_my_compress_roundtrip_bytes(self):
        """my_compress + decompress restores original bytes exactly."""
        original = b"hello world" * 50
        compressed = my_compress(original)
        decompressed = decompress(compressed)
        assert decompressed == original

    def test_my_compress_empty_bytes(self):
        """Compressing empty input works."""
        compressed = my_compress(b"")
        decompressed = decompress(compressed)
        assert decompressed == b""

    def test_my_compress_unicode(self):
        """Round-trip preserves unicode."""
        original = "日本語テスト 🎉".encode("utf-8")
        compressed = my_compress(original)
        decompressed = decompress(compressed)
        assert decompressed == original

    def test_my_compress_compression_level(self):
        """Compression level argument is accepted without error."""
        data = b"test data " * 20
        result = my_compress(data, level=1)
        assert isinstance(result, bytes)
        # Decompression must still work
        assert decompress(result) == data

    def test_my_compress_compression_level_9(self):
        """Level 9 (max) works."""
        data = b"test data " * 20
        result = my_compress(data, level=9)
        assert isinstance(result, bytes)
        assert decompress(result) == data


# =============================================================================
# Serialization / Deserialization
# =============================================================================

class TestSerialization:
    def test_serialize_deserialize_roundtrip(self):
        """serialize_compressed + deserialize_compressed restores data."""
        original = b"serialize test " * 10
        compressed = serialize_compressed(original)
        deserialized = deserialize_compressed(compressed)
        assert deserialized == original

    def test_serialize_deserialize_preserves_compressed_data(self):
        """deserialize_compressed output can be decompressed back to original."""
        original_text = "The database needs an index."
        original_bytes = original_text.encode("utf-8")
        compressed = my_compress(original_bytes)
        serialized = serialize_compressed(compressed)
        deserialized = deserialize_compressed(serialized)
        decompressed = decompress(deserialized)
        assert decompressed.decode("utf-8") == original_text

    def test_serialize_empty(self):
        """Serializing empty bytes works."""
        result = serialize_compressed(b"")
        assert isinstance(result, bytes)
        assert deserialize_compressed(result) == b""


# =============================================================================
# Token Estimation
# =============================================================================

class TestTokenEstimation:
    def test_basic_words(self):
        assert estimate_tokens("hello world") == 2

    def test_with_punctuation(self):
        assert estimate_tokens("Hello, world!") == 2

    def test_sentence_with_numbers(self):
        assert estimate_tokens("The year is 2024.") == 4

    def test_empty_string(self):
        assert estimate_tokens("") == 0

    def test_single_word(self):
        assert estimate_tokens("hello") == 1

    def test_leading_trailing_whitespace(self):
        assert estimate_tokens("  hello   world  ") == 2

    def test_unicode_words(self):
        assert estimate_tokens("日本語") == 1

    def test_mixed_content(self):
        text = "The quick-brown fox jumps over 5 lazy dogs!"
        count = estimate_tokens(text)
        assert count == 9  # The, quick, brown, fox, jumps, over, 5, lazy, dogs


# =============================================================================
# Compression Statistics
# =============================================================================

class TestStats:
    def test_get_stats_structure(self):
        """get_stats returns a dict with expected keys."""
        original = b"hello world"
        compressed = my_compress(original)
        stats = get_stats(compressed, original)
        assert isinstance(stats, dict)
        assert "original_size" in stats
        assert "compressed_size" in stats
        assert "ratio" in stats
        assert "saved_bytes" in stats
        assert "saved_percent" in stats

    def test_stats_values_compression(self):
        """get_stats returns valid structure for LZ4 output (may expand short blocks)."""
        original = b"aaaaaaaaaa"  # 10 bytes
        compressed = my_compress(original)
        stats = get_stats(compressed, original)
        assert stats["original_size"] == 10.0
        assert stats["compressed_size"] == float(len(compressed))
        # LZ4 block format has overhead; small blocks may expand
        assert stats["ratio"] > 0  # ratio is original/compressed, so if expanded ratio < 1

    def test_stats_zero_saved(self):
        """Stats handle incompressible data (ratio close to 1)."""
        import os
        original = os.urandom(100)
        compressed = my_compress(original)
        stats = get_stats(compressed, original)
        assert stats["original_size"] == 100.0
        assert stats["ratio"] <= 2.0  # should not explode


# =============================================================================
# Preprocess Text — Active Voice
# =============================================================================

class TestPreprocessText:
    """Test active voice transformation in preprocess_text."""

    # Simple single-word agents
    @pytest.mark.parametrize("input_text,expected", [
        ("The ball was thrown by John", "John threw the ball"),
        ("The cake was eaten by Mary", "Mary ate the cake"),
        ("The code was written by the developer", "the developer wrote the code"),
    ])
    def test_active_voice_single_word_agent(self, input_text, expected):
        result = preprocess_text(input_text)
        assert result == expected

    # Multi-word agents
    @pytest.mark.parametrize("input_text,expected", [
        ("The report was created by the team", "the team made the report"),
        ("The song was sung by the choir", "the choir sang the song"),
    ])
    def test_active_voice_multi_word_agent(self, input_text, expected):
        result = preprocess_text(input_text)
        assert result == expected

    def test_active_voice_irregular_verbs(self):
        """Irregular past participles map to correct simple past forms."""
        cases = [
            ("The glass was broken by the child", "the child broke the glass"),
            ("The letter was written by John", "John wrote the letter"),
            ("The view was seen by everyone", "everyone saw the view"),
        ]
        for inp, exp in cases:
            assert preprocess_text(inp) == exp, f"Failed: {inp} -> {preprocess_text(inp)} (expected {exp})"

    def test_active_voice_no_match_passthrough(self):
        """Text that doesn't match passive pattern is returned unchanged."""
        text = "John threw the ball."
        result = preprocess_text(text)
        assert result == text

    def test_active_voice_single_word_subject(self):
        """Single-word subject works: 'The ball was thrown by John'."""
        result = preprocess_text("The ball was thrown by John")
        assert result == "John threw the ball"

    def test_logical_completeness_rejects_short(self):
        """preprocess_text rejects text with fewer than 3 words."""
        with pytest.raises(Exception):  # PyValueError
            preprocess_text("Hello world")

    def test_logical_completeness_accepts_three_words(self):
        """Three-word sentences pass."""
        # "The dog barked" — but this isn't passive voice pattern
        # Use a 3-word sentence that isn't passive
        result = preprocess_text("I am here")
        assert result  # should not raise

    def test_logical_completeness_rejects_empty(self):
        with pytest.raises(Exception):
            preprocess_text("")


# =============================================================================
# Round-Trip Integration Tests
# =============================================================================

class TestRoundTrip:
    def test_preprocess_text_decompress_is_lossless(self):
        """preprocess_text output can be compressed and decompressed back."""
        original = "The ball was thrown by John."
        processed = preprocess_text(original)
        compressed = my_compress(processed.encode("utf-8"))
        decompressed = decompress(compressed).decode("utf-8")
        assert decompressed == processed


# =============================================================================
# Edge Cases
# =============================================================================

class TestEdgeCases:
    def test_very_long_text(self):
        """Long text is handled without error."""
        text = "The system processes data efficiently. " * 100
        tokens = estimate_tokens(text)
        assert tokens > 0

    def test_special_characters(self):
        """Special chars survive compression round-trip."""
        text = "Test <>&\"' chars: @#$%^&*()"
        compressed = my_compress(text.encode("utf-8"))
        decompressed = decompress(compressed).decode("utf-8")
        assert decompressed == text

    def test_newlines_tabs(self):
        """Newlines and tabs survive round-trip."""
        text = "line1\n\tline2\r\nline3"
        compressed = my_compress(text.encode("utf-8"))
        decompressed = decompress(compressed).decode("utf-8")
        assert decompressed == text


if __name__ == "__main__":
    pytest.main([__file__, "-v"])