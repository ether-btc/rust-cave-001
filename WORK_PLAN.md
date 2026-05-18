# Work Plan — rust-cave-001 Post-Audit Fixes

**Created:** 2026-05-18
**Based on:** v0.4.1 AUDIT.md findings

---

## Phase 1: Implement M-3 (estimate_tokens OnceLock)

**File:** `src/lib.rs:35-38`
**Status:** ✅ COMPLETE

**Changes:**
- Added `use std::sync::OnceLock;` to imports
- Added static `TOKEN_PATTERN: OnceLock<Regex>` before estimate_tokens
- Updated function to use `.get_or_init(|| Regex::new(...).expect(...))`
- Added doc comment: "Cached via OnceLock to avoid per-call regex compilation."

**Verification:**
- `cargo check`: PASS
- All tests: 123 passed in 0.29s

**Impact:** Eliminates per-call regex compilation overhead

---

## Phase 2: Implement M-4 (Classifier regexes caching)

**File:** `src/classifier.rs`
**Status:** ✅ COMPLETE

**Changes:**
- Added `use std::collections::HashMap;` and `use std::sync::OnceLock;` to imports
- Updated `count_sentences()`: Added static `SENTENCE_PATTERN: OnceLock<Regex>` caching
- Updated `count_pattern()`: Added `PATTERN_CACHE: OnceLock<Mutex<HashMap<String, Regex>>>` with dynamic caching
- First call compiles regex, subsequent calls return cached version

**Verification:**
- `cargo check`: PASS (0.21s)
- All tests: 123 passed in 0.29s

**Impact:** Eliminates 8+ regex compilations per classify() call (significant performance gain)

---

## Pending (Next Phases)

- M-1: `normalize_present_tense` doubled consonants (MEDIUM CORRECTNESS)
- M-2: Passive voice regex multi-word verbs (MEDIUM FEATURE)
- L-1: 88 clippy pedantic warnings (LOW STYLE)
- L-2: Docs mismatch "9 rules" vs 11 (LOW DOC)
- L-3: `resolve_pronouns` single-replacement only (LOW FEATURE)
- L-4: Missing upstream SPEC connectives `then`, `thus` (LOW SPEC)
- L-5: Duplicated stop_words list (LOW DRY)
- L-6: No ruff in CI (LOW HYGIENE)
- I-1/I-2: Upstream SPEC Rules 5 and 9 gaps (INFO STRATEGIC)
