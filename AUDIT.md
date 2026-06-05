# AUDIT REPORT — rust-cave-001
## Date: 2026-05-18 | Auditor: Hermes Agent | Branch: master@f0b9d5e

---

## Executive Summary

| Dimension | Rating | Notes |
|-----------|--------|-------|
| Correctness | ★★★☆☆ | BUG-1 confirmed; SEC-1 fix has format mismatch |
| Performance | ★★★★☆ | OnceLock caching good; classifier PATTERN_CACHE uses Mutex |
| Security | ★★★☆☆ | SEC-1 partial; decompression path uses wrong lz4 module |
| Testing | ★★★★★ | 28 Rust + 127 Python tests; all passing |
| Architecture | ★★★★☆ | Clean separation; classifier + verb_maps + error modules |

**Overall: PROCEED WITH CAUTION** — BUG-1 needs fix before production use with technical content.

---

## BUG-1: Acronym Stripping with `(?i)` Case-Insensitive Regex

**Severity: HIGH** (confirmed, not fixed)

### Findings

All content-removal functions in `src/lib.rs` that use `(?i)` case-insensitive regex:

| Line | Function | Pattern | Guard? | Acronym Risk |
|------|----------|---------|--------|-------------|
| 345 | `remove_articles` | `(?i)\b(this\|the\|a\|an)\b` | **NO** | HIGH: strips "THE" (acronym in "THE API", "THE protocol") |
| 474 | `remove_copular_be` | `(?i)\b(is\|are\|was\|were\|am\|be\|been\|being)\b` | **YES** (line 491) | LOW: correctly skips "IS", "AM", "BE" |
| 510 | `remove_intensifiers` | `(?i)\b(very\|extremely\|quite\|rather\|really\|somewhat)\b` | NO | LOW: "AND" possible edge case |
| 544 | `eliminate_connectives` | `(?i)\s*\b(because\|however\|therefore\|but\|and\|or\|...)\b,?\s*` | NO | LOW: "AND", "OR" acronyms |
| 128/145/161 | `transform_active_voice` | `(?i)\bThe\s+...` | N/A | N/A (transformation, not removal) |
| 284 | `split_into_sentences` | `(?i)\b(dr\|mr\|...)\.$` | N/A | N/A (abbreviation detection) |
| 454 | `expand_contractions` | `(?i)\b{}\b` | N/A | N/A (expansion, not removal) |

### Root Cause

`remove_copular_be` has the correct pattern (line 489-493):
```rust
.replace_all(text, |caps: &regex::Captures| {
    let word = caps.get(0).unwrap().as_str();
    // Acronym protection: skip removal if word is fully uppercase
    if word.chars().all(|c| c.is_uppercase()) {
        return word.to_string();
    }
    String::new()
})
```

`remove_articles`, `remove_intensifiers`, and `eliminate_connectives` use `replace_all(text, "")` with no per-match guard.

### Concrete False-Positive Trace

```
Input: "THE API handles requests efficiently"
Pipeline: remove_articles
Pattern: (?i)\b(this|the|a|an)\b
Match: "THE" (regex is case-insensitive, "THE" matches "the")
Output: " API handles requests efficiently"
Semantic: CORRUPTED — "THE" was an acronym (Technical Hierarchy Element), not an article
```

### Recommended Fix Pattern (apply to `remove_articles`, `remove_intensifiers`, `eliminate_connectives`)

Change from:
```rust
pattern.replace_all(text, "").to_string()
```

To:
```rust
pattern.replace_all(text, |caps: &regex::Captures| {
    let word = caps.get(0).unwrap().as_str();
    if word.chars().all(|c| c.is_uppercase()) {
        word.to_string()  // preserve acronyms
    } else {
        String::new()
    }
}).to_string()
```

---

## BUG-2: Pronoun Resolution — ALL Occurrences Replaced

**Severity: MEDIUM → FIXED**

### Fix Verification (src/lib.rs:652-672)

**Before (v0.4.0):** `break` after first pronoun match → second "it" preserved
```rust
for (j, word) in current_words.iter().enumerate() {
    ...
    if pronouns.contains(&clean.as_str()) && noun_candidates.len() >= 2 {
        pronoun_idx = Some(j);
        break;  // ← BUG: exits loop after first
    }
}
if let Some(idx) = pronoun_idx {
    ... replace at `idx` only ...
}
```

**After (v0.4.1):** Collect all indices → replace all
```rust
let pronoun_indices: Vec<usize> = current_words
    .iter()
    .enumerate()
    .filter(|(_, word)| {
        let clean = (*word).trim_end_matches(['.', ',', '!', '?']).to_lowercase();
        pronouns.contains(&clean.as_str()) && noun_candidates.len() >= 2
    })
    .map(|(j, _)| j)
    .collect();

if !pronoun_indices.is_empty() {
    ...
    .map(|(j, w)| {
        if pronoun_indices.contains(&j) {  // replaces ALL
            replacement.to_string()
        } else {
            w.to_string()
        }
    })
    ...
}
```

**Status:** ✅ CORRECTLY FIXED. All pronoun occurrences now replaced.

---

## SEC-1: Decompress Input Size Limits

**Severity: LOW → PARTIAL (correct intent, wrong lz4 module)**

### Fix Verification (src/lib.rs:31-49)

```rust
pub fn decompress(data: &[u8]) -> PyResult<Vec<u8>> {
    if data.len() < 4 {
        return Err(...);  // min 4 bytes
    }
    let uncompressed_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if uncompressed_size > 1 << 28 {  // 256 MiB limit
        return Err(...);
    }
    let decompressed = block::decompress(data, None)  // ← CONCERN
        .map_err(...)?;
    Ok(decompressed)
}
```

### Issue: Wrong lz4 Module

The code uses `lz4::block::decompress()` but the SEC-1 comment says "LZ4 format stores uncompressed size in first 4 bytes (little-endian u32)" — this describes **lz4::frame** format, not **lz4::block** format.

- `lz4::block::decompress(data, None)` takes raw LZ4 block data — the first 4 bytes are **not** a size prefix.
- `lz4::frame::decompress()` reads a proper frame header with size fields.

### What This Means

The size check reads the first 4 bytes of whatever block data is passed — this is coincidental data, not a meaningful size field. The check could:
- **Reject** valid blocks whose first 4 bytes happen to decode to >256 MiB
- **Pass** a malicious block whose first 4 bytes happen to decode to <256 MiB but decompresses to something larger

However, `lz4::block::decompress(data, None)` with `None` output limit uses lz4's internal decompression which has its own safety limits on output allocation. The fix provides some defense-in-depth but is not structurally correct.

### Recommendation

Use `lz4::frame::decompress` if frame format is expected, or document that the 4-byte check is a rough heuristic on block data.

**Status:** ⚠️ PARTIAL — intent correct, implementation checks wrong format's header.

---

## PERF-1: OnceLock Caching — Already Fixed (Confirmed)

### verb_maps.rs (lines 717-725)

```rust
static PP_CACHE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static SP_CACHE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn verb_conjugation_map() -> &'static HashMap<&'static str, &'static str> {
    PP_CACHE.get_or_init(|| PAST_PARTICIPLE_TO_SIMPLE_PAST.iter().cloned().collect())
}
pub fn present_tense_map() -> &'static HashMap<&'static str, &'static str> {
    SP_CACHE.get_or_init(|| SIMPLE_PAST_TO_PRESENT.iter().cloned().collect())
}
```

✅ **CONFIRMED FIXED** — HashMaps built once, reused via OnceLock.

### classifier.rs `count_pattern` (lines 247-268)

```rust
static PATTERN_CACHE: OnceLock<std::sync::Mutex<HashMap<String, Regex>>> = OnceLock::new();
```

⚠️ **NOTE:** Uses `Mutex<HashMap>` instead of `OnceLock<HashMap>` because the inner type (`Regex`) is not `Sync`. This is correct but has slightly more overhead than a pure `OnceLock`. Acceptable.

---

## Passive Voice Patterns — Full Analysis

### All 3 Patterns (lines 128, 145, 161)

```rust
// had_been: "The X had been V-ed by Z" → "Z had V-ed the X"
r"(?i)\bThe\s+(.+?)\s+had\s+been\s+([\w\s]+?)\s+by\s+(.+)"
// were:     "The X were V-ed by Z"     → "Z V-ed the X"
r"(?i)\bThe\s+(.+?)\s+were\s+([\w\s]+?)\s+by\s+(.+)"
// was:      "The X was V-ed by Z"       → "Z V-ed the X"
r"(?i)\bThe\s+(.+?)\s+was\s+([\w\s]+?)\s+by\s+(.+)"
```

**All use `OnceLock` static compilation** ✅

**Verb capture uses `([\w\s]+?)` (non-greedy)** ✅ — correctly handles multi-word verbs like "carried out", "set up" ✅

### `resolve_verb` Past-Participle Lookup (line 132-140)

```rust
let resolve_verb = |verb_pp: &str| -> String {
    verb_conjugations
        .get(verb_pp)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if verb_pp.ends_with("ed") && verb_pp.len() > 2 {
                verb_pp[..verb_pp.len() - 2].to_string()  // regular: "jumped" → "jump"
            } else {
                verb_pp.to_string()
            }
        })
};
```

**Issue:** For multi-word verbs captured by `([\w\s]+?)`, `verb_pp` may contain spaces (e.g., "carried out"). The `ends_with("ed")` check fails, and the fallback returns "carried out" unchanged — correct for irregular multi-word verbs, but the `verb_conjugation_map` lookup won't find multi-word entries (none exist). This is a pre-existing known limitation tracked in WORK_PLAN.md.

**Clippy fix (needless_borrow):** `verb_pp.as_str()` passed instead of `&verb_pp` ✅

---

## L-5: Duplicated `stop_words` List

**Severity: LOW** — not a bug, DRY violation

`stop_words` is defined inline in `resolve_pronouns` (line 591-621). The same list appears in `is_noun` closure as an exclusion check. No other location defines this list — not actually duplicated. **CONCERN: CLOSED (not a bug)**.

---

## Summary of Findings

| ID | Severity | Status | Finding |
|----|----------|--------|---------|
| BUG-1 | HIGH | OPEN | `remove_articles`/`remove_intensifiers`/`eliminate_connectives` strip uppercase acronyms — need same guard as `remove_copular_be` |
| BUG-2 | MEDIUM | ✅ FIXED | `resolve_pronouns` now replaces ALL pronoun occurrences |
| SEC-1 | LOW | ⚠️ PARTIAL | Decompress uses `lz4::block` but checks frame header format — format mismatch |
| PERF-1 | MEDIUM | ✅ FIXED | Verb maps cached via OnceLock |
| L-5 | LOW | CLOSED | No actual duplication found |

---

## Verification Commands

```bash
cd ~/.hermes/projects/rust-cave-001
cargo test        # 28 passed
python -m pytest # 127 passed
cargo clippy      # No issues found
cargo fmt --check # clean
gh run list       # 3 most recent: all ✅ success
```

---

*Report generated: 2026-05-18 | Auditor: Hermes Agent (MiniMax-M2.7)*

---

# Audit Cycle 2 — 2026-06-05

**Auditor:** Hermes Agent (MiniMax-M3) · **Branch:** `audit-2026-06-05` · **Commits:** e86518e, 4512531

## Executive Summary

| Dimension | Rating (was → now) | Notes |
|-----------|---------------------|-------|
| Correctness | ★★★☆☆ → ★★★★★ | I-2 (acronym word-merge), I-3 (PP/SP map ambiguity), I-7 (non-guard word-merge) all fixed; I-1 false positive correctly reverted |
| Performance | ★★★★☆ | Unchanged — OnceLock caching in place |
| Security | ★★★☆☆ → ★★★★☆ | SEC-1 re-verified (false positive); lz4 crate `prepend_size=true` puts size at start, code is correct |
| Testing | ★★★★★ | 31 → 34 Rust tests (+3 regression); 127 Python tests unchanged; ruff + mypy + clippy + fmt all clean |
| Architecture | ★★★★☆ | Unchanged |

**Overall: PRODUCTION-READY** — 5 issues identified, 4 fixed, 1 false positive correctly reverted and documented. All checks green.

## Issues Found in Cycle 1 (2026-05-29 review)

| ID | Severity | File:line | Status | Description |
|----|----------|-----------|--------|-------------|
| I-1 | HIGH (claimed) | src/lib.rs:40 | ❌ REVERTED | Reviewer claimed `decompress()` should read the LZ4 size from the LAST 4 bytes. Empirical + source verification: the `lz4` crate's `block::compress(..., prepend_size=true)` writes the size to the FIRST 4 bytes. Original code was correct. |
| I-2 | HIGH | src/lib.rs:572 | ✅ FIXED | `eliminate_connectives` acronym guard called `.trim()` on the matched span and returned the trimmed word, losing the leading/trailing spaces. Caused `"FOO AND BAR"` → `"FOOANDBAR"`. Fixed: guard now returns the full match. |
| I-3 | MEDIUM | src/verb_maps.rs:430-466 | ✅ FIXED | 34 past-participle forms (forgotten, eaten, written, etc.) were duplicated in `SIMPLE_PAST_TO_PRESENT`. Pipeline order is PP→SP→PRES, so PPs should only live in the PP→SP map. The duplicate would cause `"I have forgotten the password"` → `"I have forget the password"`. Fixed: removed all 34 entries. |
| I-4 | LOW | scripts/uninstall.sh:124 | ✅ FIXED | `error "..." && exit 1` depended on `error()` returning success. If the function ever returned non-zero, the `&&` would short-circuit and `exit 1` would never run. Fixed: replaced with `;` so `exit 1` always runs. |
| I-6 | LOW | src/error.rs:23 | ✅ FIXED | Bogus `#[allow(dead_code)]` on `into_pyerr()` — the method is used at lib.rs:788, 827. Removed the bad attribute. The remaining `#[allow(dead_code)]` on `VoiceTransformFailed`/`EmptyInput` enum variants is correct (forward-declared for future use). |

## Issues Found in Cycle 2 (2026-06-05 re-audit after Cycle 1 fixes)

| ID | Severity | File:line | Status | Description |
|----|----------|-----------|--------|-------------|
| I-7 | MEDIUM | src/lib.rs:590 | ✅ FIXED | The non-guard branch of `eliminate_connectives` was returning `""` (empty), which removed the connective AND its surrounding spaces. This caused word merging in the most common case: `"Use index because query slow"` → `"Use indexquery slow"`. The function's stated purpose is to PREVENT word merging, but its implementation caused it. Fixed: return `" "` (single space) and add `collapse_spaces` post-processing. |

## Issues Reviewed and Closed (no action needed)

| ID | Severity | File | Status | Description |
|----|----------|------|--------|-------------|
| I-5 | LOW | n/a | CLOSED | No actual duplication of `stop_words` (reviewer claim was incorrect). |
| PERF-1 | MEDIUM | n/a | CLOSED | Verb maps already cached via OnceLock (closed in 2026-05-29 audit). |
| M-2 (multi-word verb phrases) | n/a | src/lib.rs:103 | VERIFIED | Tested live: `"The ball was carried out by John"` → `"John carried out the ball"`. The regex captures multi-word phrases correctly, and the `resolve_verb` fallback returns the unchanged phrase when the map lookup fails (which IS the simple past form for regular multi-word verbs). |
| count_pattern mutex race | LOW | src/classifier.rs:270 | BENIGN | Two threads may both miss the cache and both compile the same regex. No corruption; second insert is wasted work. Not worth fixing (the `OnceLock` per-pattern refactor is a perf nit, not a bug). |

## Regression Tests Added

| Test | File | Covers |
|------|------|--------|
| `test_eliminate_connectives_acronym_guard` (2 new assertions) | src/lib.rs:935 | I-2: `"FOO AND BAR"`, `"XOR BUT Y"` — acronym guard preserves spacing |
| `test_no_past_participles_in_simple_past_map` | src/verb_maps.rs:773 | I-3: 34 PP forms must NOT be in SP→PRES |
| `test_true_simple_pasts_still_present` | src/verb_maps.rs:799 | I-3: 8 true SP forms MUST be in SP→PRES |
| `test_eliminate_connectives_no_word_merging` | src/lib.rs:1106 | I-7: 4 cases of lowercase connective removal preserving word boundaries |

## Final Validation (Cycle 3)

```
cargo test --lib        → 34 passed, 0 failed
maturin develop --release
PYTHONPATH=src python -m pytest tests/ -q
                        → 127 passed
cargo clippy --all-targets -- -D warnings
                        → clean
cargo fmt --check       → clean
ruff check tests/       → All checks passed
mypy tests/ --ignore-missing-imports
                        → Success: no issues found
maturin build --release → 879 KB manylinux wheel built
fresh venv install + pytest → 127 passed (roundtrip verified)
```

LZ4 roundtrip verified on 10 cases including empty string, 100-char and 215-char strings.

## Commits on `audit-2026-06-05`

```
4512531 fix(audit-2026-06-05): I-7 — eliminate_connectives no longer causes word merging
e86518e fix(audit-2026-06-05): apply Cycle 1 audit fixes (4 issues, 1 false positive)
1084aa6 docs: CONTINUE_HERE for v0.4.3 release (2026-06-05)  ← base
```

## Production-Readiness Verdict

**✅ READY FOR MERGE** — All issues from both audit cycles are addressed and verified. No known HIGH/MEDIUM bugs remain. The branch `audit-2026-06-05` is ready to be merged into `master` and a v0.4.4 (or v0.5.0) release tagged.

---

*Cycle 3 report appended: 2026-06-05 | Auditor: Hermes Agent (MiniMax-M3)*

---

