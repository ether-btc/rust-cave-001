# RUST-CAVE-001 — Caveman Compression in Rust

Lightweight LZ4-based text compression with Python bindings, designed for token reduction in LLM contexts.

## What It Does

Removes predictable grammar while preserving factual content — sentences get shorter without losing meaning.

**Input:**
```
The database needs an index because the queries are too slow. However, adding an index has some overhead.
```

**Output (conceptual goal):**
```
DB needs index, queries too slow. Adding index has overhead.
```

## Project Structure

```
rust-cave-001/
├── src/
│   ├── lib.rs           # Main implementation + PyO3 bindings
│   └── bin/
│       └── debug_logical.rs
├── target/
│   ├── debug/           # Debug build artifacts
│   ├── release/         # Release build (librust_cave_001.so)
│   └── maturin/         # Maturin build cache
├── .venv/              # Python 3.13 virtual environment (installed package)
├── Cargo.toml          # Rust dependencies
├── pyproject.toml      # Maturin/pip config
├── setup.py            # Legacy setup (unused)
└── PROJECT_STATUS.md   # Detailed status and issue tracker
```

## Quick Start

```bash
cd /srv/sync/projects/rust-cave-001
source .venv/bin/activate

# Test the installed module
python -c "
from rust_cave_001 import my_compress, decompress, estimate_tokens, get_stats

text = 'The database needs an index because the queries are too slow.'
data = text.encode()

compressed = my_compress(data)
stats = get_stats(compressed, data)
print(f'Original: {len(data)} bytes, Compressed: {len(compressed)} bytes')
print(f'Ratio: {stats[\"ratio\"]:.2f}x, Saved: {stats[\"saved_percent\"]:.1f}%')

# Decompress
decompressed = decompress(compressed)
print(f'Round-trip: {decompressed.decode() == text}')
"
```

## Functions

| Function | Description |
|----------|-------------|
| `my_compress(data, level=9)` | LZ4 compress bytes |
| `decompress(data)` | LZ4 decompress bytes |
| `estimate_tokens(text)` | Count words via regex |
| `get_stats(compressed, original)` | Return dict with ratio/savings |
| `serialize_compressed(data, level=9)` | Compress already-serialized data |
| `deserialize_compressed(data)` | Decompress to serialized form |
| `preprocess_text(text)` | Active voice + logical completeness check |

## Building from Source

```bash
cd /srv/sync/projects/rust-cave-001

# Clean previous builds
rm -rf target/debug target/release

# Install maturin and build
pip install maturin
maturin develop

# Or in release mode
maturin develop --release
```

**Note:** Building requires ~150MB free disk space. If you encounter "No space left on device", free up space first (see PROJECT_STATUS.md for space recovery options).

## Active Voice Transformation

The `preprocess_text` function converts passive voice to active voice:

```
"The ball was thrown by John" → "John threw the ball"
"The cake was eaten by Mary"  → "Mary ate the cake"
```

Uses a verb conjugation map for irregular past participles. The `normalize_tense` step is disabled because it corrupts output (e.g., "made" → "mak").

## Verb Conjugation Map

Built-in irregular verb transformations (past participle → simple past):

| Participle | Past | Participle | Past |
|-------------|------|-------------|------|
| thrown | threw | created | made |
| eaten | ate | written | wrote |
| seen | saw | given | gave |
| done | did | taken | took |
| built | built | drawn | drew |
| broken | broke | grown | grew |
| flown | flew | sunk | sank |
| drunk | drank | run | ran |

## Known Issues

- **Disk space**: Building requires ~150MB. Root partition on Pi fills up during compilation.
- **normalize_tense**: Disabled in `preprocess_text` — corrupts verbs like "made" → "mak". Fix exists in source, needs rebuild.
- **Decompression**: Currently a placeholder (returns data as-is). Full implementation pending.
- **Bincode serialization**: Not yet integrated with compression pipeline.

See `PROJECT_STATUS.md` for detailed issue tracking and resolution progress.

## Tech Stack

- Rust 2021 edition
- PyO3 0.24.2 (Python bindings)
- LZ4 1.0 (compression)
- Regex 1.10 (text processing)
- Maturin (Python packaging)

## Platform

Tested on Raspberry Pi 5 (arm64 / aarch64). Cross-platform compatibility testing pending.