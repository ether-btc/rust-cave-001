# CONTINUE HERE — rust-cave-001

## v0.4.1 → v0.4.2 — AUDIT FIXES COMPLETE

| Item | Value |
|------|-------|
| HEAD | `f0b9d5e` |
| Prev | `be8f3d6` (CI green before fixes) |
| Tag | [v0.4.1](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.1) |
| Rust Tests | 28 PASS ✅ |
| Python Tests | 123 PASS ✅ |
| Clippy | 0 warnings ✅ |
| cargo fmt | clean ✅ |
| CI | GREEN ✅ (3 consecutive: 26049619756, 26050043345, 26050355755) |

---

## Audit Fixes Applied This Session

### BUG-2 (MEDIUM) — resolve_pronouns replaced only first occurrence
**File:** `src/lib.rs:653-670`
**Symptom:** Sentence "It was great and it helped" → only first "it" replaced
**Fix:** Changed from `break` after first match to collecting all `pronoun_indices` via `filter + map`, then replacing all via `pronoun_indices.contains(&j)`.
**Commit:** `00f2202`

### SEC-1 (LOW) — decompress() had no input size limits
**File:** `src/lib.rs:27-49`
**Symptom:** Malicious LZ4 frame with huge declared decompressed size could allocate excessive memory.
**Fix:** Added header validation:
- Rejects inputs < 4 bytes (LZ4 frame minimum)
- Reads uncompressed_size from first 4 bytes (little-endian u32)
- Rejects sizes > 256 MiB (2^28) to prevent decompression bombs
**Commit:** `00f2202`

---

## Audit Status (All Items)

| ID | Severity | Status | Notes |
|----|----------|--------|-------|
| BUG-1 | HIGH | Open | `(?i)` on removal regexes — `remove_copular_be` has uppercase skip but other functions need review |
| BUG-2 | MEDIUM | ✅ FIXED | resolve_pronouns now replaces ALL occurrences |
| BUG-3 | MEDIUM | ✅ FIXED | Passive voice agent regex double-check for "The " prefix |
| PERF-1 | MEDIUM | ✅ FIXED | count_pattern cached via OnceLock |
| SEC-1 | LOW | ✅ FIXED | decompress() now validates size limits |

---

## CI History

| Run | SHA | Result | Notes |
|-----|-----|--------|-------|
| 26050355755 | f0b9d5e | ✅ success | docs: log BUG-2 and SEC-1 fixes |
| 26050043345 | 00f2202 | ✅ success | fix: BUG-2 replace all pronouns, SEC-1 decompress size limits |
| 26049619756 | be8f3d6 | ✅ success | docs: update CONTINUE_HERE |
| 26049391338 | dcfaca1 | ✅ success | fix(clippy): needless_borrow |
| 26049254018 | b310a62 | ❌ failure | clippy: needless_borrow |
| 26045414023 | 03cda81 | ❌ failure | cargo fmt --check |

---

## Remaining Open Items

1. **BUG-1** (HIGH): `(?i)` on content-removal regexes strips acronyms — need to audit all pipeline functions
2. **L-1**: 88 clippy pedantic warnings
3. **L-2**: Docs mismatch "9 rules" vs 11
4. **L-4**: Missing upstream SPEC connectives `then`, `thus`
5. **L-5**: Duplicated stop_words list
6. **L-6**: No ruff in CI
7. **I-1/I-2**: Upstream SPEC Rules 5 and 9 gaps
8. **crates.io publish**: needs `cargo login` token

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