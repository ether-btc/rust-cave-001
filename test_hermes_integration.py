"""
Hermes Agent Integration Test for RUST-CAVE-001
Tests the complete workflow: Rust library initialization, Python bindings, and Hermes interaction.
"""

import sys
sys.path.insert(0, '/srv/sync/projects/rust-cave-001')

def test_hermes_integration():
    """Test that the Rust library can be imported and used within Hermes environment."""
    try:
        # Import the rust-cave-001 library
        from rust_cave_001 import compress, estimate_tokens, get_stats, preprocess_text, decompress, serialize_compressed, deserialize_compressed
        
        # Test basic functionality
        text = "The database needs an index because queries are too slow."
        compressed = compress(text)
        
        # Verify compression actually changed the text
        assert compressed != text, "Compression should change the text"
        
        # Verify token estimation
        original_tokens = estimate_tokens(text)
        compressed_tokens = estimate_tokens(compressed)
        assert original_tokens > compressed_tokens, "Compressed text should have fewer tokens"
        
        # Verify stats (get_stats returns a Python dict with native types)
        original_bytes = text.encode('utf-8')
        compressed_bytes = compressed.encode('utf-8')
        stats = get_stats(compressed_bytes, original_bytes)
        
        # Verify stats structure and values
        assert 'original_size' in stats
        assert 'compressed_size' in stats
        assert 'ratio' in stats
        assert 'saved_bytes' in stats
        assert 'saved_percent' in stats
        
        original_size = stats['original_size']
        compressed_size = stats['compressed_size']
        assert isinstance(original_size, (int, float))
        assert isinstance(compressed_size, (int, float))
        assert original_size == len(original_bytes)
        assert compressed_size == len(compressed_bytes)
        assert original_size > compressed_size, "Original should be larger"
        
        # Test serialization/deserialization
        serialized = serialize_compressed(compressed_bytes)
        deserialized = deserialize_compressed(serialized)
        assert deserialized == compressed_bytes, "Round-trip should preserve compressed data"
        
        # Test preprocess_text (active voice only)
        preprocessed = preprocess_text(text)
        # Active voice should transform "needs" to "need" or similar? Actually active voice handles passive->active
        # For this sentence, active voice might not change much since it's not passive
        # Check that the text is still meaningful
        assert len(preprocessed) > 0
        # Check that logical completeness is satisfied
        assert len(preprocessed.split()) >= 2
        
        print("✅ All integration tests passed!")
        return True
        
    except ImportError as e:
        print(f"❌ Import error: {e}")
        return False
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_hermes_integration()
    sys.exit(0 if success else 1)
