# rust-cave-001 — Session Reference (May 18, 2026)

## v0.4.0 — SHIPPED

**Both blocking PRs merged and released. CI green.**

### Key References
| What | Value |
|------|-------|
| Repo | `github.com/ether-btc/rust-cave-001` |
| Latest commit | `6f80d6d` — perf(verb_maps): OnceLock caching for HashMap lookups |
| Tag | [v0.4.0](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.0) |
| PyPI | `pip install rust-cave-001==0.4.0` |
| Tests | 24 Rust — all pass ✅ |
| Clippy | 0 warnings ✅ |

### What's New in v0.4.0
- Custom error types (`CompressionError` enum)
- Abbreviation-aware sentence splitting (27 abbreviations protected)
- OnceLock caching for hot-path regexes and verb maps (performance)
- build.rs dynamic python3-config detection (fixes x86_64 CI)
- 24 Rust unit tests

### Pipeline (11 rules)
1. Sentence splitting → 2. Pronoun resolution → 3. Contraction expansion →
4. Active voice → 5. Present tense → 6. Article removal (min3) →
7. Intensifier removal (min3) → 8. Be verb removal (min2, acronym-safe) →
9. Connective removal → 10. Word limit (5) → 11. Completeness check

### Audit Findings (2026-05-18) — Open Issues
| # | Severity | Location | Issue |
|---|----------|----------|-------|
| BUG-1 | HIGH | `lib.rs` | `(?i)` on content-removal regexes can strip acronyms (IS, AM, BE, AN, BUT, OR, AND) |
| BUG-2 | MEDIUM | `lib.rs` | `resolve_pronouns` only replaces first occurrence, not all |
| BUG-3 | MEDIUM | `lib.rs` | Passive voice agent regex captures leading "The" as agent name |
| PERF-1 | MEDIUM | `classifier.rs` | `count_pattern` recompiles regex per call |
| SEC-1 | LOW | `lib.rs` | No input size limits on `decompress()` |

Full report: `wiki/audits/rust-cave-001-audit-2026-05-18-full.md`

### Next Steps (Priority Order)
1. Fix BUG-1: add `(?<![A-Z])...(?![A-Z])` uppercase boundary guards to removal regexes
2. Fix BUG-2: resolve_pronouns loop (replace all occurrences, not just first)
3. Fix BUG-3: passive voice agent regex double-checks for "The " prefix
4. Fix PERF-1: cache `count_pattern` in `static OnceLock<Regex>`
5. File GitHub issue for v2 pronoun resolution tracking
6. crates.io publish — requires `cargo login` token