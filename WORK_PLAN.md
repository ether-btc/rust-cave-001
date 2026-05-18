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

## Phase 4: Implement M-2 (Passive voice multi-word verbs)

**File:** `src/lib.rs:106-144`
**Status:** ✅ COMPLETE

**Issue:** Passive voice regex patterns only matched single-word verbs (`\w+`), missing common phrasal passives like "was carried out by" or "was set up by".

**Root Cause:** All three passive patterns used `(\w+)` for verb capture, which only matches a single word.

**Changes:**
- Extended all three passive patterns (was, were, had been) to use `([\w\s]+?)` instead of `\w+`
- Added `.trim().to_string()` in had_been pattern to normalize verb_pp with whitespace
- This allows capture of 2-3 word verb phrases (e.g., "carried out", "set up", "looked over")

**Verification:**
- Added 3 new test cases: `test_passive_multi_word_verb_carried_out`, `test_passive_multi_word_verb_set_up`, `test_passive_multi_word_verb_looked_over`
- `cargo check`: PASS
- All tests: 127 passed (up from 124)
- Multi-word verbs now correctly captured: "was carried out by" → verb phrase preserved

**Impact:** Corrects MEDIUM severity feature gap — common phrasal passive constructions now transformed

**Note:** "had been" multi-word verbs affected by separate pre-existing bug where `normalize_present_tense` converts "had" → "have" when "had" is used as auxiliary verb. Tracked separately.

---

## Pending (Next Phases)

- L-1: 88 clippy pedantic warnings (LOW STYLE)
- L-2: Docs mismatch "9 rules" vs 11 (LOW DOC)
- L-3: `resolve_pronouns` single-replacement only — ✅ FIXED (v0.4.2)
- L-4: Missing upstream SPEC connectives `then`, `thus` (LOW SPEC)
- L-5: Duplicated stop_words list (LOW DRY)
- L-6: No ruff in CI (LOW HYGIENE)
- SEC-1: No input size limits on `decompress()` — ✅ FIXED (v0.4.2)
- I-1/I-2: Upstream SPEC Rules 5 and 9 gaps (INFO STRATEGIC)
