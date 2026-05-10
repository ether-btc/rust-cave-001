#!/usr/bin/env python3
import sys
import os

# Add the directory containing the compiled library to the library path
lib_path = '/srv/sync/projects/rust-cave-001/target/release'
if lib_path not in sys.path:
    sys.path.insert(0, lib_path)

# Set LD_LIBRARY_PATH so the .so can be found
os.environ['LD_LIBRARY_PATH'] = lib_path + ':' + os.environ.get('LD_LIBRARY_PATH', '')

from rust_cave_001 import compress, decompress, estimate_tokens, get_stats

test_text = """
The database needs an index because the queries are too slow. 
However, adding an index has some overhead. 
We should test different index types to find the optimal solution.
"""

print("Original text:")
print(test_text)
print(f"\nOriginal tokens estimate: {estimate_tokens(test_text)}")

compressed = compress(test_text)
print("\nCompressed text:")
print(compressed)
print(f"Compressed tokens estimate: {estimate_tokens(compressed)}")

stats = get_stats(test_text, compressed)
print(f"\nCompression stats: {stats}")

decompressed = decompress(compressed)
print(f"\nDecompressed text:")
print(decompressed)
