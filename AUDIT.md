# rust-cave-001 v0.4.1 — Code Audit Report

**Date:** 2026-05-18
**Scope:** Full codebase (`src/lib.rs`, `src/classifier.rs`, `src/error.rs`, `src/verb_maps.rs`, `tests/test_rust_cave_001.py`, `build.rs`)
**Auditor:** Hermes Agent (automated multi-tool audit)
**Tools used:** OCR v2.1.1 (L1 structural), repomix v1.14.0, cargo clippy (pedantic+nursery), cargo audit, ruff, octocode (upstream SPEC comparison), manual line-by-line review

---

## Executive Summary

The codebase is in **good shape** — no security vulnerabilities (cargo audit: 0), no structural issues (OCR: 100/100 A+), all 28 Rust + 123 Python tests pass. The 88 clippy pedantic warnings are all style/naming, not logic bugs. The upstream SPEC comparison reveals rust-cave-001 now implements 7 of 9 upstream rules, with 2 remaining gaps (Rule 5: Preserve Specifics, Rule 9: Logical Completeness validation).

**Risk rating: LOW.** No CRITICAL or HIGH severity findings.

---

## Tool Results Summary

| Tool | Result |
|------|--------|
| OCR L1 scan | 100/100 A+, 0 issues |
| cargo audit | 0 vulnerabilities (31 crates) |
| cargo clippy (default) | PASS (after 2 format-string fixes applied) |
| cargo clippy (pedantic+nursery) | 88 style warnings (see Appendix A) |
| ruff (Python) | 1 finding (f-string without placeholders, fixed) |

---

## Findings

### MEDIUM — Correctness Issues

**M-1: `normalize_present_tense` ed-stripping produces incorrect base forms**

- **File:** `src/lib.rs:186-218`
- **Description:** The regular-verb fallback strips "ed" or "d" based on vowel checks, but produces many incorrect stems. Examples: `"stopped"` → `"stopp"` (should be `"stop"`), `"agreed"` → `"agree"` (correct), `"included"` → `"include"` (correct), `"sorted"` → `"sort"` (correct, consonant before -ted), `"fitted"` → `"fitt"` (should be `"fit"`).
- **Impact:** Present-tense normalization is lossy — double-consonant verbs are not handled. The verb_maps module covers the most common cases, so this only fires on unmapped regulars.
- **Suggested fix:** After ed-stripping, check for doubled final consonant (e.g., `"stopp"` → strip trailing doubled `p` → `"stop"`). Add a `dedup_final_consonant()` helper.
- **Severity:** MEDIUM (semantic — wrong output, but the word is still recognizable)

**M-2: Passive voice regex only matches single-word verbs**

- **File:** `src/lib.rs:105,120,135`
- **Description:** All three passive patterns use `(\w+)` for the verb capture, which matches only a single word. Phrasal passives like "was carried out by" or "was set up by" are not transformed. Multi-word past participles ("was looked down upon by") are also missed.
- **Impact:** Common passive constructions in academic/technical text pass through untransformed.
- **Suggested fix:** Change `(\w+)` to `((?:\w+\s*)+)` or similar to capture multi-word verb phrases. This is a significant regex change — test thoroughly.
- **Severity:** MEDIUM (feature gap — common patterns unhandled)

**M-3: `estimate_tokens` recompiles regex every call**

- **File:** `src/lib.rs:35-38`
- **Description:** Unlike every other function in the file, `estimate_tokens` does NOT use `OnceLock` for its `\b\w+\b` regex. Called repeatedly, this is a needless per-call allocation.
- **Impact:** Performance — the function creates and compiles a new Regex on every invocation.
- **Suggested fix:** Add `static ONCELOCK: OnceLock<Regex>` pattern, consistent with all other functions.
- **Severity:** MEDIUM (performance — trivial fix)

**M-4: Classifier `count_pattern` and `count_sentences` recompile regexes every call**

- **File:** `src/classifier.rs:238-244`
- **Description:** `count_sentences()` creates a new `Regex` on every call. `count_pattern()` creates a new `Regex` on every call. The classifier calls these ~10 times per `classify()` invocation. Each creates + compiles + drops a regex.
- **Impact:** Performance — 10+ regex compilations per classification call. The classifier is called by `compress_adaptive()`, so every adaptive compression pays this cost.
- **Suggested fix:** Cache the classifier's regexes in `OnceLock` statics, or pre-compile them in a `LazyLock<HashMap<&str, Regex>>`.
- **Severity:** MEDIUM (performance)

### LOW — Style / Maintainability

**L-1: 88 clippy pedantic+nursery warnings**

- **File:** `src/lib.rs`, `src/classifier.rs`, `build.rs`
- **Description:** See Appendix A for full breakdown. Major categories:
  - 17x `cast_lossless` (usize ↔ f64)
  - 12x `doc_markdown` (missing backticks in doc comments)
  - 11x `module_name_repetitions` (function names repeat module name)
  - 9x `missing_errors_doc` (pub fn returning Result without `# Errors` doc section)
  - 7x `uninlined_format_args` (could use inline format variables)
  - 7x `cast_possible_truncation` + 7x `cast_sign_loss` (f64 → usize)
  - 5x `redundant_closure_for_method_calls`
  - 4x `items_after_statements`
  - 2x `cloned_instead_of_copied`, `unnecessary_wraps`, `missing_const_for_fn`
  - 1x `needless_pass_by_value`, `match_same_arms`, `map_unwrap_or`
- **Suggested fix:** Run `cargo clippy --fix --allow-dirty -- -W clippy::pedantic -W clippy::nursery` to auto-fix what's possible, then manually address the rest. Or add `#![allow(clippy::pedantic, clippy::nursery)]` to lib.rs and selectively enable specific lints.
- **Severity:** LOW (style — zero functional impact)

**L-2: `compress()` doc says "9 rules" but pipeline has 11**

- **File:** `src/lib.rs:713`
- **Description:** Comment says `/// Full-pipeline compress — all 9 rules` but the pipeline now has 11 steps (added pronoun resolution + contraction expansion).
- **Suggested fix:** Update doc to "all 11 rules".
- **Severity:** LOW (documentation)

**L-3: `resolve_pronouns` only replaces first pronoun occurrence**

- **File:** `src/lib.rs:612-621`
- **Description:** The loop breaks after finding the first pronoun in each sentence. If a sentence contains multiple ambiguous pronouns ("it did it"), only the first is resolved.
- **Impact:** Low — multi-pronoun sentences with ambiguous references are rare in practice.
- **Suggested fix:** Collect all pronoun indices and replace each with the same or contextually-appropriate noun.
- **Severity:** LOW (feature gap)

**L-4: `eliminate_connectives` uses a fixed list, missing upstream SPEC connectives**

- **File:** `src/lib.rs:502-503`
- **Description:** The upstream SPEC v1.0 lists `so`, `then`, `thus` as prohibited connectives. rust-cave-001's list does not include `then` or `thus`.
- **Suggested fix:** Add `then`, `thus` to the regex pattern.
- **Severity:** LOW (SPEC compliance)

**L-5: `stop_words` list in `resolve_pronouns` duplicates lists from other functions**

- **File:** `src/lib.rs:550-595`
- **Description:** The stop_words array is defined inline in `resolve_pronouns`. It partially duplicates the words handled by `remove_articles`, `remove_intensifiers`, and `eliminate_connectives`. Any addition to those lists must be mirrored here or pronoun resolution will treat "articles" as nouns.
- **Suggested fix:** Extract to a shared constant `const STOP_WORDS: &[&str] = &[...];` at module level.
- **Severity:** LOW (DRY / maintainability)

**L-6: No Python linting in CI**

- **Description:** `tests/test_rust_cave_001.py` (939 lines) has no CI lint step. The ruff finding (now fixed) would not have been caught automatically.
- **Suggested fix:** Add a ruff step to `.github/workflows/ci.yml`.
- **Severity:** LOW (CI hygiene)

### INFO — Upstream SPEC Compliance Gap

**I-1: SPEC Rule 5 (Preserve Specifics) — not implemented**

- **Description:** The upstream SPEC requires keeping specific numbers/quantities and not replacing them with vague terms. rust-cave-001 has no explicit rule for this. In practice, the pipeline's word-limit truncation could destroy specifics (e.g., "50 million requests per second across 15 regions" → first 5 words only). This is a design-level gap, not a bug.
- **Severity:** INFO (the static ceiling at 48.7% is the real constraint — this won't move the needle alone)

**I-2: SPEC Rule 9 (Logical Completeness) — partially implemented**

- **Description:** The upstream SPEC requires every inference step to be explicit (can a reader reconstruct the full reasoning chain?). rust-cave-001 only checks ≥2 words, which is a necessary but insufficient condition. The upstream's anti-pattern examples (over-compression, telegraphic ambiguity) are not detected.
- **Severity:** INFO (the 2-word guard catches the worst cases; full Rule 9 compliance requires semantic understanding)

---

## Fixes Applied During Audit

| Fix | File | Type |
|-----|------|------|
| Inline format arg `python{ver}` | `build.rs:27` | clippy |
| Inline format arg `native={path}` | `build.rs:36` | clippy |
| Remove unnecessary `f` prefix | `tests/test_rust_cave_001.py:605` | ruff |

---

## Appendix A: Clippy Pedantic+Nursery Warning Breakdown

| Count | Lint | Category |
|-------|------|----------|
| 17 | `cast_lossless` | usize → f64, use f64 from start |
| 12 | `doc_markdown` | Backtick code identifiers in doc comments |
| 11 | `module_name_repetitions` | Fn names repeat module prefix |
| 9 | `missing_errors_doc` | `# Errors` section missing on Result-returning pub fns |
| 7 | `uninlined_format_args` | Use `format!("{var}")` not `format!("{}", var)` |
| 7 | `cast_possible_truncation` | f64 → usize may lose precision |
| 7 | `cast_sign_loss` | f64 → usize loses sign |
| 5 | `redundant_closure_for_method_calls` | Replace closure with method ref |
| 4 | `items_after_statements` | Move items before statements |
| 2 | `cloned_instead_of_copied` | Use `.copied()` not `.cloned()` for Copy types |
| 2 | `unnecessary_wraps` | Return value unnecessarily wrapped in Result |
| 2 | `missing_const_for_fn` | Could be `const fn` |
| 1 | `needless_pass_by_value` | Take &str instead of String |
| 1 | `match_same_arms` | Identical match arm bodies |
| 1 | `map_unwrap_or` | Use `map_or` instead of `map().unwrap_or()` |
| **88** | **Total** | |

---

## Test Coverage Assessment

| Area | Tests | Coverage |
|------|-------|----------|
| Passive voice (was/were/had_been) | 6 Rust + 4 Python | GOOD |
| Regular verbs | Verb map covers 192 PP entries | GOOD |
| Irregular verbs | 220 SP→Present entries | GOOD |
| Contractions | 12 basic + 6 edge cases | GOOD |
| Articles | 4 Rust + Python tests | ADEQUATE |
| Intensifiers | 3 Rust tests | ADEQUATE |
| Connectives | 4 Rust tests | ADEQUATE |
| Word limit | 3 Rust tests | ADEQUATE |
| Sentence splitting | 3 Rust tests | ADEQUATE |
| Pronoun resolution | Python tests only | ADEQUATE |
| Classifier | 8 Rust tests | GOOD |
| Adaptive compression | 5 Python tests | ADEQUATE |
| **Missing test scenarios** | Empty input to compress() | GAP |
| | Unicode/multibyte input | GAP |
| | Very long input (>10K chars) | GAP |
| | Adversarial input (all-abbreviations) | GAP |
| | Multi-sentence passive voice | GAP |

---

## Recommendation

Ship the 3 fixes, address M-1 through M-4 in a v0.4.2 maintenance release, and tackle the 88 clippy pedantic warnings as a sweep. The upstream SPEC gaps (Rules 5, 9) are strategic — they don't block the current release but should inform the self-learning framework roadmap.
