# rust-cave-001 — Session Reference (May 16, 2026)

## v0.3.0 — FULLY SHIPPED

**All 3 stages complete + post-release bugfixes + publishing.**

Stage 1: Verb maps module (94→306 PP, 147→357 PRES entries)
Stage 2: Contraction expansion (60+ forms) + copular be removal
Stage 3: Conjunction reduction (and/or added to eliminate_connectives)
5-model expert review → 4 bugs fixed, 13 tests added
PyPI v0.3.0 published, GitHub Release published

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` |
| Latest commit | `4d8136b` — post-release fixes (acronym, caching, tests) |
| Tag | [v0.3.0](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.3.0) |
| PyPI | `pip install rust-cave-001==0.3.0` |
| Tests | 108 Python / 17 Rust — all pass, clippy clean |
| Benchmark | 48.7% avg token reduction (+0.3% over v0.2.1) |

### Pipeline (11 rules)
1. Sentence splitting → 2. Pronoun resolution → 3. Contraction expansion →
4. Active voice → 5. Present tense → 6. Article removal (min3) →
7. Intensifier removal (min3) → 8. Be verb removal (min2, acronym-safe) →
9. Connective removal → 10. Word limit (5) → 11. Completeness check

### Next for v0.4.0 (if continuing)
- **"were" passive regex** — extend transform_active_voice to match plural subjects
- **"had been" passive** — past-perfect passive pattern support
- **Self-learning framework** — adaptive strategy tuning (see benchmark ceiling at 48.7%)
- **crates.io publish** — requires `cargo login` token
