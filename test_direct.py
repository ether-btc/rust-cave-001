#!/usr/bin/env python3
import sys
import os

# Add the directory containing the compiled library to the library path
lib_path = '/srv/sync/projects/rust-cave-001/target/release'
if lib_path not in sys.path:
    sys.path.insert(0, lib_path)

# Set LD_LIBRARY_PATH so the .so can be found
os.environ['LD_LIBRARY_PATH'] = lib_path + ':' + os.environ.get('LD_LIBRARY_PATH', '')

try:
    from rust_cave_001 import compress, decompress, estimate_tokens, get_stats
    print("Successfully imported rust_cave_001 module")
    
    test_text = """
    The database needs an index because the queries are too slow. 
    However, adding an index has some overhead. 
    We should test different index types to find the optimal solution.
    """
    
    compressed = compress(test_text)
    print(f"Compressed: {compressed}")
    
except ImportError as e:
    print(f"Import error: {e}")
    # List files in the directory to debug
    print("\nFiles in lib directory:")
    for f in os.listdir(lib_path):
        print(f"  {f}")
