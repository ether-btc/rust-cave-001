import sys
import os

# Add the directory containing the compiled library to the path
lib_path = '/srv/sync/projects/rust-cave-001/target/release'
if lib_path not in sys.path:
    sys.path.insert(0, lib_path)

# Set LD_LIBRARY_PATH so the .so can be found
os.environ['LD_LIBRARY_PATH'] = lib_path + ':' + os.environ.get('LD_LIBRARY_PATH', '')

# Import the Rust-compiled module directly
from rust_cave_001 import my_compress, decompress, estimate_tokens, get_stats, serialize_compressed, deserialize_compressed

test_text = "The database needs an index because the queries are too slow. However, adding an index has some overhead."
print("Original text:", test_text)
print()

compressed = my_compress(test_text.encode())
print("Compressed:", compressed)
print()

stats = get_stats(compressed, test_text.encode())
print("Compression stats:", stats)
print()

# Test serialization
serialized = serialize_compressed(compressed)
print("Serialized (binary length):", len(serialized), "bytes")
print()

# Test deserialization
deserialized = deserialize_compressed(serialized)
print("Deserialized back to compressed text:", deserialized)
print()

print("Serialization/deserialization test:", "PASS" if deserialized == compressed else "FAIL")
print()

# Test round-trip: original -> compress -> serialize -> deserialize -> decompress
roundtrip = decompress(deserialized).decode('utf-8')
print("Round-trip result:", roundtrip)
print("Original vs round-trip:", "PASS" if test_text == roundtrip else "FAIL")
