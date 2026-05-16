# rust-cave-001 — Session Reference (May 16, 2026)

## Resume Here

Code audit complete and fixes applied (v0.2.1). **Next: build the self-learning framework** — strategy selector that picks compression rules per input type.

### Quickstart
```
git clone https://github.com/ether-btc/rust-cave-001
cd rust-cave-001
source .venv/bin/activate
maturin develop --release
pytest tests/ -v        # 66 tests, all pass
python3 benchmarks/benchmark.py
```

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` (master) |
| Latest commit | `25932c6` — "fix: cargo fmt alignment in test comment" |
| Tags | v0.1.0, v0.1.1, v0.2.0, v0.2.1 |
| CI | Green — 66/66 tests, benchmarks 48.4% avg token reduction |
| Current version | **v0.2.1** (deduplicated maps, clean docs, fresh wheel) |
| Ceiling | 48.4% static — self-learning needed for 55-60% |

### What Exists
- **9 compression rules** in pipeline (`src/lib.rs`, 813 lines)
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types, LZ4, combined pipeline
- **66 passing tests** (`tests/test_rust_cave_001.py`) — includes direct `normalize_present_tense` tests (26 new)
- **ATTRIBUTION.md** — credits upstream SPEC repos
- **Verb maps**: 94 unique entries (transform_active_voice) + 147 unique entries (normalize_present_tense)

### Latest Audit Findings Addressed (v0.2.1)
- ✅ Deduplicated verb conjugation maps (removed 27 duplicate entries)
- ✅ Bumped Cargo.toml/pyproject.toml v0.2.0 → v0.2.1
- ✅ De-hermesified descriptions in pyproject.toml
- ✅ Fixed build.rs DEP_PYO3_ABI3 check (PY312 → PY310)
- ✅ Updated BENCHMARKS.md version (v0.1.0 → v0.2.1)
- ✅ Updated README.md verb count (~60 → ~100)
- ✅ Added 26 direct tests for normalize_present_tense
- ✅ Removed dead code comment in preprocess_text

### Next Steps (priority order)
1. **Self-learning framework design & build** — strategy selector + auto-benchmark oracle
2. **Publish to PyPI** — needs API token from pypi.org/manage/account/token/
3. **MCP server integration** — expose as MCP tool
4. **Add tests for resolve_pronouns** across ambiguous/unambiguous cases
5. **Expand verb map** — 94 → 150+ unique entries
6. **Update CI** — move from actions/cache@v4 (Node 20 deprecation) to v5

### Upstream Repos to Watch
| Repo | Why | Absorb When |
|------|-----|-------------|
| wilpel/caveman-compression | Direct SPEC upstream, MLM/LLM modes | Framework phase |
| JuliusBrussee/caveman (37K ⭐) | Market validation, /caveman-compress | Post-framework |
| claudioemmanuel/squeez (~2K ⭐) | MCP server architecture, Rust hooks | Framework phase |
