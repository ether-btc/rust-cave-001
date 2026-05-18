# rust-cave-001 — Session Reference (May 18, 2026)

## v0.4.0 — FULLY SHIPPED

**Custom error types, abbreviation-aware sentence splitting, fully passing tests.**

Stage 1: Build fix (libpython linking via PYO3_BUILD_EXTENSION_MODULE guard) + 23 Rust tests
Stage 2: Error types + abbreviation-aware sentence splitting + build.rs fallback
Stage 3: Error types wired into pipeline + changelog
Stage 4: Maturin build fix + v0.4.0 release prep

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` |
| Latest commit | `df70f59` — build.rs hardcodes python3.13 for aarch64 linking fix |
| Tag | [v0.4.0](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.0) |
| PyPI | `pip install rust-cave-001==0.4.0` |
| Tests | 108 Python / 24 Rust — all pass, clippy clean (1 dead_code warning on unused error variants) |
| Benchmark | 48.7% avg token reduction (v0.3.0 baseline) |

### Pipeline (11 rules)
1. Sentence splitting → 2. Pronoun resolution → 3. Contraction expansion →
4. Active voice → 5. Present tense → 6. Article removal (min3) →
7. Intensifier removal (min3) → 8. Be verb removal (min2, acronym-safe) →
9. Connective removal → 10. Word limit (5) → 11. Completeness check

### v0.4.0 additions
- Custom `CompressionError` enum (TooShort, VoiceTransformFailed, EmptyInput, PipelineError)
- Abbreviation-aware sentence splitting (27 common abbreviations protected)
- 23 new Rust unit tests (total: 24 Rust / 108 Python)
- Dynamic python version detection in build.rs

### Build system fix
- `build.rs` now hardcodes `python3.13` linking on aarch64 (container python3→3.11 but target is 3.13)
- `PYO3_BUILD_EXTENSION_MODULE` guard prevents libpython linking during maturin wheel builds

### Next for v0.4.1 (if continuing)
- **"were" passive regex** — extend transform_active_voice to match plural subjects
- **"had been" passive** — past-perfect passive pattern support
- **Self-learning framework** — adaptive strategy tuning (benchmark ceiling at 48.7%)
- **crates.io publish** — requires `cargo login` token
