# rust-cave-001 — Session Status (May 15, 2026)

## What Was Done
1. **Benchmark suite added** — `benchmarks/benchmark.py` with:
   - NLP compression across 9 text types (technical, academic, conversational, dialogue, mixed, etc.)
   - LZ4 binary compression on various data types
   - Combined pipeline (compress → LZ4) end-to-end
   - 7 sanity checks (all passing)
2. **BENCHMARKS.md created** — detailed results table
3. **README updated** — benchmarks section, removed "No benchmark suite" limitation

## Benchmark Results (RPi 5, aarch64)
- **NLP compress()** — avg 48.4% token reduction, 7.4ms per call
- **LZ4 my_compress()** — up to 136x on repeated data, 3.88x on JSON, 7-14µs
- **Combined pipeline** — up to 3.05x total ratio (conversational text)
- All 7 sanity checks: ✅ PASSED

## Next Steps
1. **Publish to PyPI** — needs API token: https://pypi.org/manage/account/token/
2. **Expand verb map** — from 60 to 200+ irregular verbs
3. **Wire Caveman into Mnemosyne** — auto-detect instead of env var (Issue #3)
4. **Consider zstd** for binary compression (better ratio than LZ4 for JSON)
