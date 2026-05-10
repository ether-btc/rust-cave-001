"""
Rust-based Caveman Compression library
"""
import os
import sys

# Add the directory containing the compiled library to the path
lib_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'target', 'release')
if lib_path not in sys.path:
    sys.path.insert(0, lib_path)

# Set LD_LIBRARY_PATH so the .so can be found
os.environ['LD_LIBRARY_PATH'] = lib_path + ':' + os.environ.get('LD_LIBRARY_PATH', '')

# Import the Rust-compiled module
from rust_cave_001 import my_compress, decompress, estimate_tokens, get_stats, preprocess_text, serialize_compressed, deserialize_compressed, compress
