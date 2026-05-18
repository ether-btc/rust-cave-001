# CONTINUE HERE — rust-cave-001

## v0.4.1 — SHIPPED + CI GREEN

| Item | Value |
|------|-------|
| Commit | `dcfaca1` |
| Tag | [v0.4.1](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.1) |
| PyPI | Wheel built: `dist/rust_cave_001-0.4.1-cp310-abi3-manylinux_2_34_aarch64.whl` — needs manual upload (no token on this machine) |
| Rust Tests | 28 PASS ✅ |
| Python Tests | 123 PASS ✅ |
| Clippy | 0 warnings ✅ |
| CI | GREEN (all jobs pass) ✅ |

### What's New in v0.4.1
- **Plural passive voice** — `"were V-ed by"` regex transforms to active
- **Past-perfect passive voice** — `"had been V-ed by"` regex transforms to active
- **Double-article fix** — all passive patterns no longer produce `"the The X"`
- 4 Rust + 4 Python tests for the new patterns

### Bug Fixes Applied (May 18)
- **BUG-1** (HIGH): Acronym stripping — `(?i)` prefix on content-removal regexes stripped uppercase acronyms (IS, AM, BE, AN, BUT, OR, AND). Status: Fixed in v0.4.1 via regex boundary adjustments. No regression on test suite.
- **BUG-2** (MEDIUM): `resolve_pronouns` only replaced first occurrence. Status: Not yet fixed.
- **BUG-3** (MEDIUM): Passive voice agent regex captured leading "The" as agent name. Status: Fixed in v0.4.1.
- **PERF-1** (MEDIUM): `count_pattern` recompiled regex per call. Status: Fixed (static OnceLock cache).
- **SEC-1** (LOW): No input size limits on `decompress()`. Status: Not yet fixed.

### Local Fixes Applied (May 18)
- `b310a62` style: rustfmt fixes for lib.rs and classifier.rs (indentation on static OnceLock patterns)
- `dcfaca1` fix(clippy): needless_borrow on verb_pp in had_been passive transform

### CI History (May 18)
- `26045414023` FAIL: `cargo fmt --check` — pre-existing formatting issues on origin/master
- `26049254018` FAIL: `cargo clippy` — needless_borrow lint caught after rustfmt re-indented the closure
- `26049391338` SUCCESS: All jobs pass ✅ (latest commit dcfaca1)

### Next Priorities
1. **BUG-2 fix**: `resolve_pronouns` loop — replace all occurrences, not just first
2. **SEC-1 fix**: Add input size limits on `decompress()`
3. PyPI upload — needs token or GitHub Actions trusted publishing (OIDC)
4. crates.io publish — needs `cargo login` token
5. Self-learning framework — adaptive strategy tuning (benchmark ceiling 48.7%)
6. Broader passive patterns: "is being V-ed", "has been V-ed", "will be V-ed"

### Key Files
| File | Path |
|------|------|
| Main lib | `src/lib.rs` |
| Classifier | `src/classifier.rs` |
| Error types | `src/error.rs` |
| Verb maps | `src/verb_maps.rs` |
| Build script | `build.rs` |
| Python tests | `tests/test_rust_cave_001.py` |
| CI workflow | `.github/workflows/ci.yml` |