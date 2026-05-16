# rust-cave-001 — Session Reference (May 16, 2026)

## Resume Here — Stage 2 Complete (Contraction Expansion + "Be" Removal)

Stage 1: **Verb map expansion** — completed.
Stage 2: **Contraction expansion + "be" verb removal** — completed.
- `expand_contractions()`: 60+ English contractions expanded (n't, 's, 're, 've, 'll, 'd, informal)
- `remove_copular_be()`: Removes is/are/was/were/am/be/been/being with 2-word safety guard
- Both rules wired into `apply_caveman_rules()` pipeline and all classifier strategies
- All 17 Rust tests, 82 Python tests pass, clippy clean

**Next: Stage 3** — Conjunction reduction (compress "and"/"but"/"or" clauses).

### Quickstart
```
git clone https://github.com/ether-btc/rust-cave-001
cd rust-cave-001
source .venv/bin/activate
maturin develop --release
pytest tests/ -v        # 82 tests, all pass
python3 benchmarks/benchmark.py
```

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` (master) |
| Latest commit | `238d23a` — feat: wire classifier into compress_adaptive |
| Tags | v0.1.0, v0.1.1, v0.2.0, v0.2.1 |
| CI | Green — Rust clippy clean, 17/17 Rust tests, 82/82 Python tests |
| Current version | **v0.2.1** (working on v0.3.0) |
| Verb maps | `src/verb_maps.rs` — 192 pp→sp, 220+ sp→pres |
| Ceiling | 48.4% static — adaptive strategy via `compress_adaptive()` is self-learning MVP |

### What Exists
- **9 compression rules** in pipeline (`src/lib.rs`)
- **Input classifier** (`src/classifier.rs`) — 13-heuristic text type detection + greeting detection
- **Verb maps module** (`src/verb_maps.rs`) — static verb conjugation maps
- **Adaptive compression API** — `compress_adaptive()` auto-selects rule subset per input type
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types + strategy comparison oracle
- **82 passing tests** — all pass, 17 Rust unit tests

### Stage Roadmap

| Stage | Feature | Status |
|-------|---------|--------|
| 1 | Verb map expansion (94→192 pp, 147→220 pres) | ✅ Complete |
| 2 | Contraction expansion ("don't"→"do not") + "be" verb removal | 🔲 Next |
| 3 | Conjunction reduction ("and"/"but"/"or" clause compression) | 🔲 Planned |
| 4 | v0.3.0 release — tag, PyPI, crates.io | 🔲 Planned |

### Notes
- "dealt with"/"carried out" phrasal verbs in passive voice are a pre-existing regex limitation
- PyPI publish requires `MATURIN_PYPI_TOKEN` env var
