#!/usr/bin/env python3
import sys
sys.path.insert(0, '/srv/sync/projects/rust-cave-001/target/release')

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
