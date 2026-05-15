# rust-cave-001

[![CI](https://github.com/ether-btc/rust-cave-001/actions/workflows/ci.yml/badge.svg)](https://github.com/ether-btc/rust-cave-001/actions)
[![crates.io](https://img.shields.io/crates/v/rust-cave-001.svg)](https://crates.io/crates/rust-cave-001)
[![Benchmarks](BENCHMARKS.md)](BENCHMARKS.md)

A Rust + PyO3 library that compresses natural language text to reduce LLM token count while preserving factual content.

## What It Does

Caveman Compression is a lightweight text compression technique that removes predictable grammar while preserving facts. It applies a pipeline of rules: sentence splitting, word limiting, connective elimination, active-voice transformation, intensifier removal, article removal, and logical-completeness filtering.

**Example:**

```
Input:
"The database needs an index because the queries are too slow. However, adding an index has some overhead."

Output:
"DB needs index queries slow. Adding index overhead."
```

Token count: 18 → 8 (~56% reduction)

## Installation

### From source

```bash
git clone https://github.com/ether-btc/rust-cave-001.git
cd rust-cave-001
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop --release
```

### Python usage

```python
from rust_cave_001 import compress, estimate_tokens

text = "The database needs an index because the queries are too slow."
compressed = compress(text)
print(f"Original tokens: {estimate_tokens(text)}")
print(f"Compressed tokens: {estimate_tokens(compressed)}")
# Original tokens: 11
# Compressed tokens: 5
```

## API Reference

### `compress(text: str) -> str`

Applies the full Caveman Compression pipeline to `text`. Returns the compressed string, or raises `ValueError` if the result lacks logical completeness (fewer than 2 words).

### `preprocess_text(text: str) -> str`

Converts passive voice to active voice. Returns the transformed string, or raises `ValueError` if fewer than 2 words.

```python
from rust_cave_001 import preprocess_text
result = preprocess_text("The ball was thrown by John")
# "John threw the ball"
```

### `estimate_tokens(text: str) -> int`

Estimates token count using word-boundary regex. Useful for measuring compression ratio.

### `my_compress(data: bytes, level: int = 9) -> bytes`

Compresses raw bytes using LZ4 high-compression mode.

### `decompress(data: bytes) -> bytes`

Decompresses LZ4-compressed bytes. Use after `my_compress` for a round-trip.

### `get_stats(compressed: bytes, original: bytes) -> dict`

Returns compression statistics:

```python
{
    "original_size": 55.0,
    "compressed_size": 35.0,
    "ratio": 1.57,
    "saved_bytes": 20.0,
    "saved_percent": 36.4
}
```

### `serialize_compressed(data: bytes, level: int = 9) -> bytes`

Applies bincode serialization then LZ4 compression.

### `deserialize_compressed(data: bytes) -> bytes`

Decompresses then deserializes: the inverse of `serialize_compressed`.

## The 7 Compression Rules

Applied in order by `compress()`:

1. **Sentence splitting** — Split on `.`, `!`, `?`, then process each sentence independently
2. **Word limit** — Truncate to 5 words per sentence; split on comma first if possible
3. **Connective elimination** — Remove `because`, `however`, `therefore`, `but` (case-insensitive)
4. **Active voice transform** — Convert passive ("was written by") to active ("wrote") using a verb conjugation map with 60+ irregular verbs
5. **Intensifier removal** — Remove `very`, `extremely`, `quite`, `rather`, `really`, `somewhat`
6. **Article removal** — Remove `the`, `a`, `an`, `this` (unless removal would leave fewer than 3 words)
7. **Logical completeness** — Reject output with fewer than 2 words

## Building from Source

Prerequisites: Python 3.10+, Rust 1.70+

```bash
git clone https://github.com/ether-btc/rust-cave-001.git
cd rust-cave-001
pip install maturin
maturin develop
```

For a release build:

```bash
maturin develop --release
```

## Running Tests

```bash
pytest tests/ -v
```

## Tech Stack

- Rust 2021 edition
- PyO3 0.24.2 (Python bindings)
- LZ4 1.0 (compression)
- Regex 1.10 (text processing)
- Maturin (Python packaging)

## Benchmarks

See [BENCHMARKS.md](BENCHMARKS.md) for detailed performance data across 9 text types and LZ4 binary compression benchmarks.

**TL;DR:** Average token reduction of 48-55% across typical texts, call time ~7.4ms on RPi 5 (aarch64).

## Known Limitations

- Verb conjugation map covers ~60 irregular verbs; regular verbs fall back to stripping the "ed" suffix
- Two-word sentences after processing are rejected as logically incomplete
- Not designed for code, structured data, or non-English text

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions are welcome.

## License

MIT License. See [LICENSE](LICENSE).