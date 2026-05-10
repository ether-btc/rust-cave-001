import sys
import os
sys.path.insert(0, '/srv/sync/projects/rust-cave-001/target/release')
os.environ['LD_LIBRARY_PATH'] = '/srv/sync/projects/rust-cave-001/target/release:' + os.environ.get('LD_LIBRARY_PATH', '')

from rust_cave_001 import compress

test = "However, this is a test."
print(f"Input: {test}")
compressed = compress(test)
print(f"Compressed: {compressed}")
