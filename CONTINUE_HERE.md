# rust-cave-001 — Session Reference (May 18, 2026)

## v0.4.0 — BLOCKED — CI FAILING on Both Open PRs

**Full audit completed 2026-05-18** — see `wiki/audits/rust-cave-001-audit-2026-05-18-full.md` for findings.

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` |
| Latest commit | `2e6d072` — .gitignore: add repomix_digest.md + .ocrrc.yml |
| Tag | [v0.4.0](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.0) |
| PyPI | `pip install rust-cave-001==0.4.0` |
| Tests | 119 Python / 24 Rust — all pass ✅ |
| Clippy | 0 warnings ✅ |
| OCR Score | 81/100 PASSED ✅ |
| Benchmark | 48.7% avg token reduction (v0.3.0 baseline) |

### Open PRs (both CI FAILING)
- **#5** `fix/build-rs-python-detection` — dynamic python3-config but `manual_strip` clippy error in build.rs line 36 (`&part[2..]`)
- **#6** `perf/verb-map-oncelock-caching` — OnceLock caching for verb maps (PASSES tests, FAILS CI due to build.rs)

### Root Cause: build.rs hardcodes python3.13
`build.rs` was rewritten to hardcode `python3.13` linking for aarch64 runners, but x86_64 GitHub Actions uses `libpython3.10.so.1.0`. Both PRs fail with: `rust-lld: error: unable to find library -lpython3.13`

**Fix needed:** Restore dynamic python3-config approach (commit `4621ab2`) with `strip_prefix("-L")` instead of `&part[2..]` to avoid clippy `manual_strip` lint.

### Pipeline (11 rules)
1. Sentence splitting → 2. Pronoun resolution → 3. Contraction expansion →
4. Active voice → 5. Present tense → 6. Article removal (min3) →
7. Intensifier removal (min3) → 8. Be verb removal (min2, acronym-safe) →
9. Connective removal → 10. Word limit (5) → 11. Completeness check

### Audit Findings (2026-05-18)
- **BLOCKER:** build.rs hardcodes python3.13 — breaks x86_64 CI
- **HIGH:** `(?i)` flag on content-removal regexes can strip acronyms (IS, AM, BE, AN, BUT, OR, AND)
- **MEDIUM:** `resolve_pronouns` only replaces first pronoun occurrence (not all)
- **MEDIUM:** Passive voice agent regex can capture leading "The" as agent name
- **MEDIUM:** `count_pattern` in classifier.rs recompiles regex per call
- **LOW:** No input size limits on decompress()
- Full report: `wiki/audits/rust-cave-001-audit-2026-05-18-full.md`

### Next Steps (Priority Order)
1. Fix build.rs — unblock both PRs
2. Fix BUG-2: resolve_pronouns loop (replace all occurrences, not just first)
3. Fix BUG-1: add `(?<![A-Z])...(?![A-Z])` uppercase boundary guards to removal regexes
4. Cache classifier.rs density patterns in `static OnceLock<Regex>`
5. File GitHub issue for v2 pronoun resolution tracking
6. crates.io publish — requires `cargo login` token