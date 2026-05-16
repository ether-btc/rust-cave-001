# Benchmarks

Benchmark results for rust-cave-001 v0.3.0.

```bash
python3 benchmarks/benchmark.py
```

## NLP Compression (`compress()`)

The Caveman Rules pipeline across 9 text types. Measured on aarch64 (Raspberry Pi 5).

| Text Type | Chars | → | Compressed | Reduction | Tokens | → | Compressed | Reduction | Time |
|---|---|---|---|---|---|---|---|---|---|
| Technical (short) | 61 | → | 32 | 47.5% | 11 | → | 5 | **54.5%** | 2.79ms |
| Technical (paragraph) | 369 | → | 147 | 60.2% | 54 | → | 22 | **59.3%** | 11.11ms |
| Conversational | 260 | → | 78 | 70.0% | 43 | → | 14 | **67.4%** | 8.58ms |
| Academic/Dense | 436 | → | 134 | 69.3% | 55 | → | 15 | **72.7%** | 8.37ms |
| Dialogue/Chat | 196 | → | 35 | 82.1% | 39 | → | 9 | **76.9%** | 5.57ms |
| Mixed (code+numbers) | 260 | → | 99 | 61.9% | 43 | → | 17 | **60.5%** | 8.32ms |
| Already minimal | 21 | → | 21 | 0.0% | 4 | → | 4 | 0.0% | 2.68ms |
| Repetitive | 179 | → | 107 | 40.2% | 36 | → | 20 | 44.4% | 11.04ms |
| Short sentences | 41 | → | 41 | 0.0% | 9 | → | 9 | 0.0% | 8.03ms |

**Summary:** Average token reduction **48.4%** (54.5% on texts with >4 tokens).
Average call time **7.4ms** on RPi 5 (aarch64).

## LZ4 Binary Compression (`my_compress`)

| Data Type | Size (B) | Compressed (B) | Ratio | Saved | Time |
|---|---|---|---|---|---|
| Repeated text (2KB) | 1,560 | 32 | **48.75x** | 97.9% | 7.6µs |
| Repeated text (10KB) | 7,800 | 57 | **136.84x** | 99.3% | 8.4µs |
| Random bytes (1KB) | 1,024 | 1,033 | 0.99x | -0.9% | 13.7µs |
| JSON-like (2KB) | 912 | 235 | 3.88x | 74.2% | 9.7µs |
| UTF-8 text (1KB) | 1,350 | 65 | 20.77x | 95.2% | 7.5µs |
| Small data (100B) | 100 | 15 | 6.67x | 85.0% | 7.2µs |

## Combined Pipeline: `compress()` → `my_compress()`

Applying NLP compression then LZ4 in sequence, measured end-to-end.

| Text | Original | After NLP | After LZ4 | Total Ratio | Total Time |
|---|---|---|---|---|---|
| Technical (short) | 61B | 32B | 38B | 1.61x | 2.9ms |
| Technical (paragraph) | 369B | 147B | 150B | 2.46x | 11.2ms |
| Conversational | 262B | 80B | 86B | **3.05x** | 8.8ms |

## Notes

- LZ4 has block overhead (~15B per block), so small blocks may expand on first call but decompress correctly.
- Random/incompressible data may have a tiny overhead from LZ4 block header.
- Timings are on a Raspberry Pi 5 (aarch64, Cortex-A76). Expect ~2-4x faster on x86_64.
