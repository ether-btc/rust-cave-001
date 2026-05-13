# rust-cave-001 — Session Status (May 13, 2026)

## Session Summary
Discovered Caveman compression is wired into Mnemosyne consolidation pipeline for the first time.

## What Was Done
1. **Integration with Mnemosyne**:
   - Caveman pre-compression added to `summarize_memories()` via `MNEMOSYNE_USE_CAVEMAN` env var
   - Used on RPi 5 with Qwen2.5-1.5B local LLM (2048 ctx)
   - Prevents context window overflow on large memory summaries

2. **GitHub Issues Filed**:
   - Issue #3: Caveman integration tracking https://github.com/ether-btc/rust-cave-001/issues/3
   - Referenced in upstream Mnemosyne PR #114 and Issue #116

## Audit Findings
- LZ4 compression ratio for JSON: 43.6% (worse than gzip at 32.3%)
- NLP text compression (compress()): 35-60% token reduction — this is where it shines
- No benchmark suite exists
- No Python wheel published (maturin develop only)
- Verb conjugation map covers ~60 irregular verbs

## Known Limitations (from README)
- Two-word sentences are rejected as logically incomplete
- Not designed for code, structured data, or non-English text
- Regular verbs fall back to stripping "ed" prefix

## Next Steps
1. Build and publish Python wheel to PyPI (cargo + PyO3 wheel)
2. Add benchmark suite for compression ratio measurement
3. Expand verb conjugation map (60 → 200+ verbs)
4. Consider zstd for binary compression (better than LZ4 for JSON)
5. Auto-detection: instead of env var, auto-use if importable
