import sys
import os
sys.path.insert(0, '/srv/sync/projects/rust-cave-001/target/release')
os.environ['LD_LIBRARY_PATH'] = '/srv/sync/projects/rust-cave-001/target/release:' + os.environ.get('LD_LIBRARY_PATH', '')

from rust_cave_001 import split_sentences

test_text = """The database needs an index because the queries are too slow. However, adding an index has some overhead. We should test different index types to find the optimal solution."""

sentences = split_sentences(test_text)
print(f"Number of sentences: {len(sentences)}")
for i, s in enumerate(sentences):
    print(f"Sentence {i+1}: '{s}'")
