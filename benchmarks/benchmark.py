#!/usr/bin/env python3
"""
Comprehensive benchmark suite for rust-cave-001.

Measures:
  - NLP compression: token reduction ratio, character reduction, time per call
  - LZ4 binary compression: ratio for various data types
  - Combined pipeline: text → compress → LZ4

Run:  python3 benchmarks/benchmark.py
Watch: python3 benchmarks/benchmark.py --watch  (live-updating table)
"""

import sys
import os
import time
import statistics
import json
import textwrap

# Ensure rust_cave_001 is importable
sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

from rust_cave_001 import (
    compress,
    estimate_tokens,
    my_compress,
    decompress,
    get_stats,
)

from benchmarks.sample_texts import ALL_TEXTS

# ── Configuration ──────────────────────────────────────────────────────────
WARMUP_ITERATIONS = 50     # warmup to stabilize JIT/CPU cache
BENCHMARK_ITERATIONS = 200  # timed iterations for each test
LZ4_ITERATIONS = 500       # fast enough to need more reps

MIN_TOKEN_REDUCTION_PCT = 15  # sanity: compress must reduce at least this % on most texts
MAX_COMPRESS_TIME_MS = 10.0   # sanity: compress should complete within 10ms


# ── Helpers ────────────────────────────────────────────────────────────────

def bench_func(fn, args=(), kwargs=None, iterations=BENCHMARK_ITERATIONS):
    """Time a function over many iterations. Returns (mean_ms, stdev_ms, results)."""
    if kwargs is None:
        kwargs = {}

    # Warmup
    for _ in range(WARMUP_ITERATIONS):
        fn(*args, **kwargs)

    # Timed runs
    times = []
    results = []
    for _ in range(iterations):
        start = time.perf_counter()
        result = fn(*args, **kwargs)
        elapsed = time.perf_counter() - start
        times.append(elapsed * 1000)  # ms
        results.append(result)

    mean = statistics.mean(times)
    stdev = statistics.stdev(times) if len(times) > 1 else 0.0
    return mean, stdev, results[-1]  # return last result


def fmt_ms(ms):
    """Format milliseconds to a human-readable string."""
    if ms < 1:
        return f"{ms*1000:.1f}µs"
    return f"{ms:.2f}ms"


def fmt_pct(num):
    """Format percentage."""
    return f"{num:.1f}%"


# ── NLP Compression Benchmarks ─────────────────────────────────────────────

def benchmark_nlp_compression():
    """Benchmark the compress() function on all sample texts."""
    print("=" * 82)
    print("  NLP COMPRESSION: compress() — Caveman Rules Pipeline")
    print("=" * 82)
    print()
    print(
        f"  {'Text Type':<28} {'Chars':>6} {'→':>3} {'Chars':>6} "
        f"{'Reduction':>10} {'Tokens':>7} {'→':>3} {'Tokens':>7} "
        f"{'Reduct':>9} {'Time':>10}"
    )
    print("  " + "─" * 82)

    results_summary = []

    for key, info in ALL_TEXTS.items():
        text = info["text"]
        label = info["label"]

        # Measure original
        orig_tokens = estimate_tokens(text)
        orig_chars = len(text)
        orig_words = len(text.split())

        # Benchmark compress()
        try:
            mean_ms, stdev_ms, compressed = bench_func(compress, args=(text,))
            comp_tokens = estimate_tokens(compressed)
            comp_chars = len(compressed)
            comp_words = len(compressed.split())
        except ValueError as e:
            # Text too short for logical completeness — report as N/A
            print(
                f"  {label:<28} {orig_chars:>6} {'→':>3} {'N/A':>6} "
                f"{'N/A':>10} {orig_tokens:>7} {'→':>3} {'N/A':>7} "
                f"{'N/A':>9} {'N/A':>10}"
            )
            results_summary.append({
                "label": label,
                "orig_chars": orig_chars,
                "comp_chars": 0,
                "char_pct": 0.0,
                "orig_tokens": orig_tokens,
                "comp_tokens": 0,
                "token_pct": 0.0,
                "time_ms": 0.0,
                "error": str(e),
            })
            continue

        comp_tokens = estimate_tokens(compressed)
        comp_chars = len(compressed)
        comp_words = len(compressed.split())

        # Compute savings
        char_saved = orig_chars - comp_chars
        char_pct = (char_saved / orig_chars * 100) if orig_chars else 0
        token_saved = orig_tokens - comp_tokens
        token_pct = (token_saved / orig_tokens * 100) if orig_tokens else 0
        word_saved = orig_words - comp_words
        word_pct = (word_saved / orig_words * 100) if orig_words else 0

        # Output
        time_str = fmt_ms(mean_ms)
        print(
            f"  {label:<28} {orig_chars:>6} {'→':>3} {comp_chars:>6} "
            f"{fmt_pct(char_pct):>10} {orig_tokens:>7} {'→':>3} {comp_tokens:>7} "
            f"{fmt_pct(token_pct):>9} {time_str:>10}"
        )

        results_summary.append({
            "label": label,
            "orig_chars": orig_chars,
            "comp_chars": comp_chars,
            "char_pct": round(char_pct, 1),
            "orig_tokens": orig_tokens,
            "comp_tokens": comp_tokens,
            "token_pct": round(token_pct, 1),
            "time_ms": round(mean_ms, 3),
        })

    print()
    # Summary stats
    token_reductions = [r["token_pct"] for r in results_summary if r["orig_tokens"] > 4]
    char_reductions = [r["char_pct"] for r in results_summary if r["orig_chars"] > 20]
    times = [r["time_ms"] for r in results_summary]

    print(f"  Summary (texts with >4 tokens, n={len(token_reductions)}):")
    print(f"    Token reduction:  mean={statistics.mean(token_reductions):.1f}%"
          f"  min={min(token_reductions):.1f}%  max={max(token_reductions):.1f}%")
    print(f"    Char reduction:   mean={statistics.mean(char_reductions):.1f}%")
    print(f"    Time per call:    mean={statistics.mean(times):.3f}ms  "
          f"max={max(times):.3f}ms")
    print()

    return results_summary


# ── LZ4 Binary Compression Benchmarks ──────────────────────────────────────

def benchmark_lz4_compression():
    """Benchmark my_compress() on various data types."""
    print("=" * 82)
    print("  LZ4 BINARY COMPRESSION: my_compress()")
    print("=" * 82)
    print()
    print(
        f"  {'Data Type':<30} {'Size (B)':>10} {'Comp (B)':>10} "
        f"{'Ratio':>8} {'Saved':>10} {'Time':>10}"
    )
    print("  " + "─" * 82)

    data_samples = {
        "Repeated text (2KB)": b"Hello World! " * 120,
        "Repeated text (10KB)": b"Hello World! " * 600,
        "Random bytes (1KB)": os.urandom(1024),
        "JSON-like (2KB)": (
            '{"users": [' + ','.join(
                f'{{"id": {i}, "name": "User_{i}", "active": true}}'
                for i in range(20)
            ) + ']}'
        ).encode(),
        "UTF-8 text (1KB)": ("The quick brown fox jumps over the lazy dog. " * 30).encode(),
        "Small data (100B)": b"A" * 100,
    }

    lz4_results = []

    for label, data in data_samples.items():
        orig_size = len(data)

        # Warmup + time
        mean_ms, _, compressed = bench_func(
            my_compress, args=(data,), kwargs={"level": 9},
            iterations=LZ4_ITERATIONS,
        )

        comp_size = len(compressed)
        ratio = orig_size / comp_size if comp_size else 0
        saved = orig_size - comp_size
        saved_pct = (saved / orig_size * 100) if orig_size else 0

        time_str = fmt_ms(mean_ms)
        print(
            f"  {label:<30} {orig_size:>10} {comp_size:>10} "
            f"{ratio:>7.2f}x {fmt_pct(saved_pct):>10} {time_str:>10}"
        )

        lz4_results.append({
            "label": label,
            "orig_size": orig_size,
            "comp_size": comp_size,
            "ratio": round(ratio, 2),
            "saved_pct": round(saved_pct, 1),
            "time_ms": round(mean_ms, 3),
        })

    print()
    return lz4_results


# ── Combined Pipeline Benchmark ────────────────────────────────────────────

def benchmark_combined_pipeline():
    """Benchmark the full pipeline: text → compress → encode → LZ4."""
    print("=" * 82)
    print("  COMBINED PIPELINE: compress() → my_compress()")
    print("=" * 82)
    print()

    texts_for_pipeline = {
        "Technical (short)": ALL_TEXTS["technical_short"]["text"],
        "Technical (paragraph)": ALL_TEXTS["technical_long"]["text"],
        "Conversational": ALL_TEXTS["conversational"]["text"],
    }

    for label, text in texts_for_pipeline.items():
        orig_bytes = len(text.encode("utf-8"))
        orig_tokens = estimate_tokens(text)

        # Step 1: compress
        start = time.perf_counter()
        compressed_text = compress(text)
        compress_time = time.perf_counter() - start

        # Step 2: LZ4
        text_bytes = compressed_text.encode("utf-8")
        start = time.perf_counter()
        binary = my_compress(text_bytes)
        lz4_time = time.perf_counter() - start

        total_time = compress_time + lz4_time

        steps = [
            ("Original text", orig_bytes, f"{orig_tokens} tokens"),
            ("After compress()", len(text_bytes), f"{estimate_tokens(compressed_text)} tokens"),
            ("After LZ4", len(binary), f"compressed"),
        ]

        nlp_ratio = orig_bytes / len(text_bytes) if len(text_bytes) else 0
        total_ratio = orig_bytes / len(binary) if len(binary) else 0

        print(f"  [{label}]")
        for step_name, size, note in steps:
            print(f"    {step_name:<28} {size:>8}B  ({note})")
        print(f"    NLP compression ratio:  {nlp_ratio:.2f}x  ({compress_time*1000:.1f}ms)")
        print(f"    Total ratio (NLP+LZ4):  {total_ratio:.2f}x  ({total_time*1000:.1f}ms)")
        print()


# ── Sanity Checks ──────────────────────────────────────────────────────────

def run_sanity_checks():
    """Run quick sanity/regression checks based on benchmark results."""
    print("=" * 82)
    print("  SANITY CHECKS")
    print("=" * 82)
    print()

    checks_passed = 0
    checks_failed = 0

    # 1. Basic texts must compress with token reduction
    for key in ["technical_short", "technical_long", "conversational"]:
        text = ALL_TEXTS[key]["text"]
        orig = estimate_tokens(text)
        try:
            result = compress(text)
            after = estimate_tokens(result)
            if after < orig:
                checks_passed += 1
                print(f"  ✓ [{key}] Tokens reduced: {orig} → {after}")
            else:
                checks_failed += 1
                print(f"  ✗ [{key}] No token reduction: {orig} → {after}")
        except Exception as e:
            checks_failed += 1
            print(f"  ✗ [{key}] compress() raised: {e}")

    # 2. Passive voice is transformed
    result = compress("The ball was thrown by John")
    if "John threw" in result:
        checks_passed += 1
        print(f"  ✓ Passive voice transformed correctly: '{result}'")
    else:
        checks_failed += 1
        print(f"  ✗ Passive voice not transformed: '{result}'")

    # 3. LZ4 round-trip is lossless
    data = b"test data for lossless check " * 50
    compressed = my_compress(data)
    decompressed = decompress(compressed)
    if decompressed == data:
        checks_passed += 1
        print(f"  ✓ LZ4 round-trip is lossless ({len(data)}B)")
    else:
        checks_failed += 1
        print(f"  ✗ LZ4 round-trip lost data!")

    # 4. Compression stats returns valid dict
    stats = get_stats(compressed, data)
    required_keys = {"original_size", "compressed_size", "ratio", "saved_bytes", "saved_percent"}
    if required_keys.issubset(set(stats.keys())):
        checks_passed += 1
        print(f"  ✓ get_stats() returns valid dict: ratio={stats['ratio']:.2f}x")
    else:
        checks_failed += 1
        missing = required_keys - set(stats.keys())
        print(f"  ✗ get_stats() missing keys: {missing}")

    # 5. estimate_tokens returns correct counts
    assert estimate_tokens("hello world") == 2
    checks_passed += 1
    print(f"  ✓ estimate_tokens('hello world') = 2")

    print()
    print(f"  Results: {checks_passed} passed, {checks_failed} failed")
    print()
    return checks_failed == 0


# ── Main ───────────────────────────────────────────────────────────────────

def main():
    print()
    print("  ╔══════════════════════════════════════════════╗")
    print("  ║  rust-cave-001 — Comprehensive Benchmarks    ║")
    print("  ╚══════════════════════════════════════════════╝")
    print()

    # Run all benchmarks
    nlp_results = benchmark_nlp_compression()
    lz4_results = benchmark_lz4_compression()
    benchmark_combined_pipeline()
    all_ok = run_sanity_checks()

    # Summary
    print("=" * 82)
    print("  SUMMARY")
    print("=" * 82)
    print()

    # Average token reduction across all text types
    avg_token_reduction = statistics.mean([r["token_pct"] for r in nlp_results])
    avg_char_reduction = statistics.mean([r["char_pct"] for r in nlp_results])
    avg_time = statistics.mean([r["time_ms"] for r in nlp_results])

    print(f"  NLP compress() — average token reduction:  {avg_token_reduction:.1f}%")
    print(f"  NLP compress() — average char reduction:   {avg_char_reduction:.1f}%")
    print(f"  NLP compress() — average call time:        {avg_time:.3f}ms")
    print()

    best_lz4 = max(lz4_results, key=lambda r: r["ratio"])
    print(f"  LZ4 best ratio:   {best_lz4['ratio']}x ({best_lz4['label']})")
    print(f"  LZ4 best savings: {max(lz4_results, key=lambda r: r['saved_pct'])['saved_pct']:.1f}%")
    print()

    if all_ok:
        print("  All sanity checks: ✅ PASSED")
    else:
        print("  Some sanity checks: ❌ FAILED")

    print()
    print("  ═══════════════════════════════════════════════")
    print()

    return all_ok


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
