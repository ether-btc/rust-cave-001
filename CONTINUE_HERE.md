# rust-cave-001 — Session Reference (May 16, 2026)

## v0.3.0 Complete

All 3 stages implemented + bug fixes from 5-model expert review + docs/publishing:

- Stage 1: Verb maps module (306 pp→sp, 357 sp→pres entries)
- Stage 2: Contraction expansion (60+ forms) + copular be removal
- Stage 3: Conjunction reduction (and/or added to eliminate_connectives)
- 4 bugs fixed: created→made mapping, cause→because corruption, pipeline ordering, acronym collision
- OnceLock regex caching: 63→1 regex compilation in expand_contractions
- 108 Python tests, 17 Rust tests, clippy clean, 48.7% avg token reduction

### Quickstart
```
git clone https://github.com/ether-btc/rust-cave-001
cd rust-cave-001
source .venv/bin/activate
maturin develop --release
pytest tests/ -v        # 108 tests, all pass
python3 benchmarks/benchmark.py
```

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` (master) |
| Latest commit | `0909dd1` — v0.3.0 full release |
| Tags | v0.3.0, v0.2.1, v0.2.0, v0.1.1, v0.1.0 |
| Release | https://github.com/ether-btc/rust-cave-001/releases/tag/v0.3.0 |
| CI | Green — Rust clippy clean, 17/17 Rust tests, 108/108 Python tests |
| Current version | **v0.3.0** (released) |
| Verb maps | `src/verb_maps.rs` — 306 pp→sp, 357 sp→pres (100% cross-map coverage) |
| Ceiling | 48.7% static — 11 rules, tested against 9 text types |
| Benchmark delta | +0.3% over v0.2.1 baseline (48.4%) |

### What Exists
- **11 compression rules** in pipeline (`src/lib.rs`)
- **Input classifier** (`src/classifier.rs`) — 13-heuristic text type detection + greeting detection
- **Verb maps module** (`src/verb_maps.rs`) — static verb conjugation maps with unit tests
- **Adaptive compression API** — `compress_adaptive()` auto-selects rule subset per input type
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types + strategy comparison oracle
- **108 passing Python tests** + 17 Rust unit tests

### Pipeline (11 rules, in order)
1. Sentence splitting
2. Pronoun resolution
3. Contraction expansion (60+ forms)
4. Active voice transformation (306 verb map)
5. Present tense normalization (357 verb map)
6. Article removal (min 3-word guard)
7. Intensifier removal (min 3-word guard)
8. Copular "be" verb removal (min 2-word guard, acronym-protected)
9. Conjunction/connective removal (because/however/therefore/but/and/or)
10. Word limit (max 5 words)
11. Logical completeness check (min 2 words)

### Known Limitations
- "dealt with"/"carried out" phrasal verbs in passive voice are a pre-existing regex limitation (single-word capture after "was")
- "were" and "had been" passive forms are not captured by the regex (pre-existing, plural subjects)
- PyPI publish: `maturin publish --skip-existing` with token
- crates.io publish: `cargo publish` after login
