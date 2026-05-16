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
| Latest commit | `ce9088c` — "feat: add input text classifier for adaptive compression strategies" |
| Tags | v0.1.0, v0.1.1, v0.2.0, [v0.2.1](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.2.1) |
| PyPI | [![PyPI](https://img.shields.io/pypi/v/rust-cave-001.svg)](https://pypi.org/project/rust-cave-001/) |
| CI | Green — 66/66 tests, benchmarks 48.4% avg token reduction |
| Current version | **v0.2.1** (deduplicated maps, clean docs, fresh wheel, input classifier built) |
| Ceiling | 48.4% static — self-learning framework component (classifier) built, strategy selector next |
### What Exists
- **9 compression rules** in pipeline (`src/lib.rs`, 813 lines)
- **Input classifier** (`src/classifier.rs`, 353 lines) — 13-heuristic text type detection
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types, LZ4, combined pipeline
- **66 passing tests** (`tests/test_rust_cave_001.py`) — includes direct `normalize_present_tense` tests (26 new)
- **ATTRIBUTION.md** — credits upstream SPEC repos
- **Verb maps**: 94 unique entries (transform_active_voice) + 147 unique entries (normalize_present_tense)

### Latest Audit Findings Addressed (v0.2.1)
- ✅ Deduplicated verb conjugation maps (removed 27 duplicate entries)
- ✅ Bumped Cargo.toml/pyproject.toml v0.2.0 → v0.2.1
- ✅ De-hermesified pyproject.toml description for public consumption
- ✅ Fixed build.rs DEP_PYO3_ABI3 check (PY312 → PY310)
- ✅ Updated BENCHMARKS.md version reference (v0.1.0 → v0.2.1)
- ✅ Updated README.md verb count (~60 → ~100)
- ✅ Added 26 direct tests for normalize_present_tense (irregular, regular, same-form, capitalization)
- ✅ Removed dead code comment in preprocess_text
- ✅ CI formatting fixed (`cargo fmt` pass on build.rs + lib.rs)

### Latest Feature (v0.2.1+)
- ✅ **Input text classifier** (`src/classifier.rs`) — detects technical, conversational, academic, dialogue, minimal, mixed text types using 13 heuristic dimensions. Exposed as Python-callable `classify_text()` and `recommended_strategy_for_text()`.

### Next Steps (priority order)
1. **Wire classifier into compress()** — use `recommended_strategy()` to select rule subset per input type
2. **Auto-benchmark oracle** — measure which strategy wins for each text domain
3. **Publish to PyPI** — needs API token from pypi.org/manage/account/token/
4. **MCP server integration** — expose as MCP tool (reference: squeez architecture)
5. **Add Python tests for classifier** — test classify_text() and recommended_strategy_for_text()
6. **Expand verb map** — 94 → 150+ unique entries in transform_active_voice

### Upstream Repos to Watch
| Repo | Why | Absorb When |
|------|-----|-------------|
| wilpel/caveman-compression | Direct SPEC upstream, MLM/LLM modes | Framework phase |
| JuliusBrussee/caveman (37K ⭐) | Market validation, /caveman-compress | Post-framework |
| claudioemmanuel/squeez (~2K ⭐) | MCP server architecture, Rust hooks | Framework phase |
