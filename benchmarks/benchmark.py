#!/usr/bin/env python3
"""
Benchmark suite for RUST-CAVE-001 Caveman Compression
Measures performance and compression ratios on various text samples.
"""
import time
import json
from pathlib import Path
from rust_cave_001 import compress, estimate_tokens, get_stats

def benchmark_compression(texts, iterations=100):
    """Benchmark compression performance and ratios."""
    results = []
    
    for name, text in texts.items():
        print(f"\nBenchmarking: {name}")
        print(f"  Original size: {len(text)} chars, {estimate_tokens(text)} tokens")
        
        # Warm-up
        compress(text)
        
        # Measure performance
        start_time = time.perf_counter()
        for _ in range(iterations):
            compressed = compress(text)
        end_time = time.perf_counter()
        
        elapsed = end_time - start_time
        avg_time_ms = (elapsed / iterations) * 1000
        
        # Get compression stats
        compressed_text = compress(text)
        compressed_bytes = compressed_text.encode('utf-8')
        stats = get_stats(compressed_bytes, text.encode('utf-8'))
        
        # Convert stats to Python dict (get_stats returns a Python dict)
        stats_dict = {
            "original_size": stats["original_size"],
            "compressed_size": stats["compressed_size"],
            "ratio": stats["ratio"],
            "saved_bytes": stats["saved_bytes"],
            "saved_percent": stats["saved_percent"],
        }
        
        result = {
            "name": name,
            "original_tokens": estimate_tokens(text),
            "compressed_tokens": estimate_tokens(compressed_text),
            "compression_ratio": stats_dict["ratio"],
            "saved_percent": stats_dict["saved_percent"],
            "avg_compression_time_ms": avg_time_ms,
            "original_chars": len(text),
            "compressed_chars": len(compressed_text),
        }
        
        results.append(result)
        print(f"  Compressed: {result['compressed_chars']} chars, {result['compressed_tokens']} tokens")
        print(f"  Compression ratio: {result['compression_ratio']:.2f}x")
        print(f"  Saved: {result['saved_percent']:.1f}%")
        print(f"  Avg time: {avg_time_ms:.3f} ms per iteration")
    
    return results

def load_sample_texts():
    """Load sample texts for benchmarking."""
    samples_dir = Path("benchmarks/samples")
    samples_dir.mkdir(exist_ok=True)
    
    # Create or load sample texts
    samples = {
        "short_sentence": "The quick brown fox jumps over the lazy dog.",
        "medium_paragraph": (
            "The quick brown fox jumps over the lazy dog. The quick brown fox jumps "
            "over the lazy dog. The quick brown fox jumps over the lazy dog."
        ),
        "long_paragraph": (
            "In a distant galaxy, far beyond the reaches of our own Milky Way, "
            "there exists a civilization unlike anything we've ever known. They "
            "communicate through complex patterns of light and sound, and their "
            "technology is so advanced that it appears as magic to our primitive "
            "minds. This is a story about one of their explorers who crash-landed "
            "on Earth millions of years ago and how they influenced the development "
            "of early human civilization."
        ),
        "technical_text": (
            "The database needs an index because queries are too slow. The query "
            "planner is not using the optimal execution plan. We need to analyze "
            "the query patterns and add appropriate indexes to improve performance."
        ),
        "email_thread": (
            "Subject: Project Update\n\nHi team,\n\nHere's a quick update on our current projects:\n\n1. Project A: On track for Q3 delivery\n2. Project B: Facing some technical challenges\n3. Project C: Resource allocation review needed\n\nLet me know if you have any questions.\n\nBest,\nJohn\n\n---\n\nSubject: Re: Project Update\n\nHi John,\n\nThanks for the update. Regarding Project B, can we schedule a call to discuss the technical challenges?\n\nAlso, add this to the agenda for tomorrow's meeting.\n\nThanks,\nSarah\n\n---\n\nSubject: Re: Project Update\n\nHi Sarah,\n\nSure, let's schedule a call for Friday at 2 PM. I'll send a calendar invite.\n\nRegarding the meeting, I've added it to the agenda.\n\nThanks,\nJohn"
        ),
    }
    
    # Save samples to files for reproducibility
    for name, text in samples.items():
        (samples_dir / f"{name}.txt").write_text(text, encoding='utf-8')
    
    return samples

def main():
    print("=" * 60)
    print("RUST-CAVE-001 BENCHMARK SUITE")
    print("=" * 60)
    
    # Load sample texts
    texts = load_sample_texts()
    
    # Run benchmark
    results = benchmark_compression(texts, iterations=1000)
    
    # Save results to JSON
    results_file = Path("benchmarks/results.json")
    results_file.parent.mkdir(exist_ok=True)
    results_file.write_text(json.dumps(results, indent=2))
    
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    for result in results:
        print(f"\n{result['name']}:")
        print(f"  Tokens: {result['original_tokens']} -> {result['compressed_tokens']}")
        print(f"  Compression: {result['saved_percent']:.1f}% reduction")
        print(f"  Time: {result['avg_compression_time_ms']:.3f} ms/iteration")
    
    print("\n" + "=" * 60)
    print(f"Results saved to: {results_file}")
    print("=" * 60)

if __name__ == "__main__":
    main()