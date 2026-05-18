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
