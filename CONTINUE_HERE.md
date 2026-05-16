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
| CI | Green — 82/82 tests, benchmarks 48.4% avg token reduction |
| Current version | **v0.2.1** (deduplicated maps, clean docs, adaptive compression operational) |
| Ceiling | 48.4% static — adaptive strategy via `compress_adaptive()` is self-learning framework MVP |
### What Exists
- **9 compression rules** in pipeline (`src/lib.rs`, 842 lines)
- **Input classifier** (`src/classifier.rs`, 349 lines) — 13-heuristic text type detection
- **Adaptive compression API** — `compress_adaptive()` auto-selects rule subset per input type
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types + strategy comparison oracle
- **82 passing tests** (`tests/test_rust_cave_001.py`) + classifier + adaptive tests
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
- ✅ **Input text classifier** (`src/classifier.rs`) — detects 6 text types using 13 heuristic dimensions
- ✅ **Adaptive compression** (`compress_adaptive()`) — wires classifier into pipeline with per-type strategy selection
- ✅ **Benchmark oracle** — compares full vs adaptive across all 9 text types; 0 regressions
- ✅ **6 direct adaptive tests** — pronouns preserved in conversational, passthrough for minimal, numbers preserved in technical

### Next Steps (priority order)
1. **Expand verb map** — 94 → 150+ unique entries in transform_active_voice (~60 missing irregular verbs)
2. **Publish v0.3.0 to PyPI** with adaptive compression and expanded verb map
3. **crates.io publish** — `cargo publish` for Rust-native use without Python
4. **MCP server integration** — expose as MCP tool (reference: squeez architecture)

### Upstream Repos to Watch
| Repo | Why | Absorb When |
|------|-----|-------------|
| wilpel/caveman-compression | Direct SPEC upstream, MLM/LLM modes | Framework phase |
| JuliusBrussee/caveman (37K ⭐) | Market validation, /caveman-compress | Post-framework |
| claudioemmanuel/squeez (~2K ⭐) | MCP server architecture, Rust hooks | Framework phase |
