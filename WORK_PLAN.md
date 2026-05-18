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

## Phase 3: Implement M-1 (normalize_present_tense doubled consonants)

**File:** `src/lib.rs:219-234`
**Status:** ✅ COMPLETE

**Issue:** The `normalize_present_tense` function incorrectly handled verbs with doubled consonants before "ed" (e.g., "stopped" → "stopp", "slipped" → "slipp", "grabbed" → "grabb").

**Root Cause:** The default "ed" stripping logic removed the "ed" suffix but did not check for doubled consonants in the stem.

**Changes:**
- Added doubled consonant detection logic in the default "ed" → "" stripping path
- Checks if last two characters of stem are the same consonant
- If true, removes one consonant before returning (e.g., "stopp" → "stop", "slipp" → "slip")
- Pattern: [consonant][same-consonant]ed → remove one consonant
- Examples: stopped→stop, slipped→slip, grabbed→grab, planned→plan, mapped→map

**Verification:**
- Added comprehensive test case `test_doubled_consonants` to `tests/test_rust_cave_001.py`
- Tests 12 doubled-consonant verbs + 4 regular verbs for regression
- `maturin build --release`: PASS (12.36s)
- `pip install --force-reinstall`: PASS
- All tests: 124 passed in 0.14s (up from 123)

**Impact:** Corrects MEDIUM severity bug affecting present-tense conversion of regular verbs with doubled consonants

---

## Pending (Next Phases)

- M-2: Passive voice regex multi-word verbs (MEDIUM FEATURE)
- L-1: 88 clippy pedantic warnings (LOW STYLE)
- L-2: Docs mismatch "9 rules" vs 11 (LOW DOC)
- L-3: `resolve_pronouns` single-replacement only (LOW FEATURE)
- L-4: Missing upstream SPEC connectives `then`, `thus` (LOW SPEC)
- L-5: Duplicated stop_words list (LOW DRY)
- L-6: No ruff in CI (LOW HYGIENE)
- I-1/I-2: Upstream SPEC Rules 5 and 9 gaps (INFO STRATEGIC)
