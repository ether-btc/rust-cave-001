## [0.6.1] - 2026-07-16

### Changed
- **Rebuild with current toolchain** — wheel rebuilt with rustc 1.96.0 / maturin (2026-07-16). No source changes vs v0.6.0; the version bump tracks the republish.
- **Smaller wheel** — the manylinux `aarch64` wheel dropped from 1.06 MB → 890 KB (~17% smaller). The compiled `.so` went from 3.18 MB (unstripped) → 2.23 MB (stripped). Same functionality; debug symbols are no longer shipped in the binary distribution.

### Fixed
- **PyPI/Git version sync** — v0.6.0 had been published to PyPI on 2026-07-08 but the working-tree version bump to 0.6.1 was left uncommitted, so the git tag, CHANGELOG, and GitHub Release lagged behind PyPI. v0.6.1 closes that gap: source, tag, and PyPI are now in sync.

### Validation
- `cargo test --lib` → 43 passed
- `pytest tests/` → 170 passed (143 core + 27 MCP server)
- `cargo clippy --all-targets -- -D warnings` → clean
- `cargo fmt --check` → clean
- Local wheel sha256 matches PyPI-published wheel sha256 (wheel + sdist verified)

## [0.6.0] - 2026-06-19

### Added
- **MCP server** — new `caveman-mcp` entry point exposes Caveman Compression as Model Context Protocol tools (`caveman_compress`, `caveman_compress_batch`, `caveman_classify`, `caveman_estimate_tokens`, `caveman_stats`). Compatible with any MCP client (Hermes Agent, Claude Desktop, etc.).
- Optional `[mcp]` extra: `pip install rust-cave-001[mcp]`.
- 27 Python tests covering the MCP server tools and protocol behaviour.

### Security
- **SEC-1**: `compress()` validates the input header and uses explicit constants.
- **SEC-2**: `decompress()` validates the input header, rejects zero-size payloads, and uses an explicit constant for the decompressed-size limit. Regression tests added.
- **SEC-3**: `compress()` enforces an input size limit (DoS prevention).
- Poisoned mutex in the classifier pattern cache is now handled gracefully.

## [0.5.0] - 2026-06-18

### Fixed
- **RUSTSEC-2026-0176 + RUSTSEC-2026-0177**: Upgraded pyo3 from 0.24.2 to 0.29 to fix two security advisories published 2026-06-11:
  - RUSTSEC-2026-0176: Out-of-bounds read in PyList/PyTuple iterator nth/nth_back
  - RUSTSEC-2026-0177: Missing Sync bound on PyCFunction::new_closure closures
  - CI cargo-audit job had been failing since 2026-06-16 due to these advisories

### Changed
- **pyo3 0.29 API migration**:
  - Python::with_gil -> Python::attach (semantic equivalent, renamed in 0.29)
  - PyObject -> Py<PyAny> (old type alias removed in 0.29; explicit generic required)

### Fixed
- **Regression: my_compress signature attribute** - Restored #[pyo3(signature = (data, level = 9))] which was accidentally dropped in commit 0e41988 when rewriting doc comments. Without this attribute, pyo3 treats the level parameter as a required positional argument instead of a keyword argument with default, causing 11 Python tests to fail.

### Lint/Quality
- **Clippy pedantic warnings driven to zero** - Fixed 88 clippy::pedantic warnings across 7 files (59 auto-fixed, 29 manual).

### Validation
- cargo clippy --all-targets -- -D warnings -> clean
- cargo test --lib -> 38 passed
- pytest tests/ -> 129 passed

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2026-06-05

### Changed
- **PyPI re-upload of v0.4.2** — same code as v0.4.2 (BUG-1 acronym guard, BUG-2 pronoun resolution, SEC-1 decompress size limits), rebuilt and republished as a fresh release on PyPI. The 0.4.1 → 0.4.3 version jump reflects the rebuild rather than new functionality.

## [0.4.2] - 2026-05-18

### Fixed
- **BUG-1: acronym stripping** — `remove_articles`, `remove_intensifiers`, and `eliminate_connectives` used `(?i)` case-insensitive regex but lacked the uppercase guard that `remove_copular_be` already had. Added per-match closure guard to preserve fully-uppercase words (acronyms like "THE", "ANDROID", "XOR"). Added 3 regression tests.

## [0.4.1] - 2026-05-18

### Added
- **Plural passive voice** (`were V-ed by`) — regex pattern transforms plural passive constructions to active voice. e.g. "The balls were thrown by John" → "John threw the balls".
- **Past-perfect passive voice** (`had been V-ed by`) — regex pattern for past-perfect passive. e.g. "The documents had been signed by the manager" → "the manager had signed the documents".
- **4 Rust tests** for new passive patterns (were, were+irregular, had_been, had_been+irregular).
- **4 Python tests** for new passive patterns via the full `compress()` pipeline.

### Fixed
- **Passive voice double-article bug** — the regex captured "The" in the subject group, producing "John threw the The ball". Fixed by matching but not capturing the leading "The".
- **BUG-2: pronoun resolution** — `resolve_pronouns` only replaced the first occurrence; now replaces ALL pronoun occurrences in a sentence. Changed from break-after-first to collecting all indices via filter+map.
- **SEC-1: decompress size limits** — `decompress()` now validates the LZ4 frame header size prefix before decompression. Rejects inputs <4 bytes and decompressed sizes >256 MiB to prevent decompression bombs.

## [0.4.0] - 2026-05-18

### Added
- **Custom error types** (`src/error.rs`) — `CompressionError` enum with 4 variants: TooShort, VoiceTransformFailed, EmptyInput, PipelineError. Includes `Display` + `std::error::Error` impls.
- **Abbreviation-aware sentence splitting** — `split_into_sentences()` now recognizes 27 common abbreviations (Dr., U.S.A., e.g., Jan., etc.) to prevent false sentence boundaries.
- **Rust unit tests** — expanded from 3 to 24 tests covering all pipeline functions: expand_contractions, remove_intensifiers, eliminate_connectives, enforce_word_limit, split_into_sentences, abbreviation protection, and edge cases (unicode, multi-contraction, possessive vs contraction).
- **build.rs dynamic linking** — detects Python version at build time instead of hardcoding 3.13. Graceful fallback when python3 is unavailable.

### Changed
- Error messages for logical completeness failures now use `CompressionError::TooShort` instead of raw `PyValueError`, providing more specific error context.
- `split_into_sentences()` uses `OnceLock`-cached regex for abbreviation matching (compiled once, not per-call).

## [0.3.0] - 2026-05-16

### Added
- **Verb map module** (`src/verb_maps.rs`) — dedicated static conjugation maps extracted from inline code. PP→SP map expanded from 94→306 entries (3.2×), SP→PRES expanded from 147→357 entries (2.4×).
- **Contraction expansion** — `expand_contractions()` expands 60+ English contractions (n't, 's, 're, 've, 'll, 'd, informal) as first pipeline step.
- **Copular "be" removal** — `remove_copular_be()` strips is/are/was/were/am/be/been/being with safety guard.
- **Conjunction reduction** — `and`/`or` added to `eliminate_connectives()` alongside because/however/therefore/but.
- **Classifier: greeting detection** — conversational classifier now detects "hey/hi/hello/thanks/please" as strong conversational signal.
- **Classifier: dialogue tightening** — dialogue guard now requires `word_count < 15` to prevent false classification.

### Fixed
- **16 missing PRES map entries** — passive-to-active verbs like "had" (→have), "dug" (→dig), and 14 same-form no-change verbs (beat→beat, bet→bet, bid→bid, etc.) now properly convert to present tense.
- **Classifier benchmark samples** — "Hey! How is everything going with the new system?" correctly classified as conversational.
- **Classifier dialogue/conversational fragmentation** — conversational text with short sentences no longer falsely classifies as dialogue.

### Changed
- Pipeline expanded to 11 rules (was 9): contraction expansion → active voice → present tense → be removal → article removal → intensifier removal → connective elimination → word limit → completeness check.
- All classifier strategies updated to include `expand_contractions` and `remove_copular_be` for appropriate text types.

## [0.2.1] - 2026-05-15

### Fixed
- `normalize_present_tense` ed-stripping: off-by-one in vowel detection (included→includ, sorted→sorte)
- Added 21 e-drop verb entries to map (agreed→agree, merged→merge, cached→cache, etc.)
- Tightened eeed guard to allow "agreed" → "agree" while still blocking "speed" → "spe"
- Added vowel-before-ded/ted check to prevent false e-drop on consonant patterns

## [0.2.0] - 2026-05-15

### Added
- **Present tense normalization** — `normalize_present_tense()` (SPEC Rule 4) converts past-tense verbs to present tense using a reverse conjugation map of 100+ verbs. Un-dead-coded from `#[allow(dead_code)]`.
- **Pronoun resolution** — `resolve_pronouns()` (SPEC Rule 8) replaces ambiguous pronouns (`it`, `they`, `them`) with preceding noun when 2+ candidates exist.
- **ATTRIBUTION.md** — Properly credits wilpel/caveman-compression SPEC, JuliusBrussee/caveman, claudioemmanuel/squeez, and all third-party crate dependencies.
- **`normalize_present_tense()` exposed** as a Python-callable function.

### Changed
- Pipeline now applies 9 rules (was 7): pronoun resolution → active voice → present tense → articles → intensifiers → connectives → word limit → completeness check.
- Cargo.toml: v0.2.0, added authors and description.
- pyproject.toml: v0.2.0.
- README: Updated rules from 7 → 9, added SPEC attribution link.

### Fixed
- Benchmark sanity check: accepts both past and present tense passive voice output.

## [0.1.1] - 2026-05-15

### Added
- Benchmark suite (benchmarks/benchmark.py) — measures NLP compression ratios, LZ4 binary compression, and combined pipeline performance across 9 text types
- BENCHMARKS.md with detailed results and performance characteristics
- Sample texts dataset (benchmarks/sample_texts.py) for reproducible benchmarking

### Changed
- README: Added benchmarks section, removed "No benchmark suite" known limitation

## [0.1.0] - 2026-05-12

### Added
- Initial public release
- LZ4 compression and decompression (`my_compress`, `decompress`)
- Token estimation via word-boundary regex (`estimate_tokens`)
- Compression statistics (`get_stats`)
- Bincode serialization pipeline (`serialize_compressed`, `deserialize_compressed`)
- Passive-to-active voice transformation (`preprocess_text`, `transform_active_voice`)
- Full Caveman Compression pipeline (`compress`) applying 7 rules:
  1. Sentence splitting
  2. Word limit (2-5 words per sentence)
  3. Connective elimination
  4. Active voice transformation
  5. Intensifier removal
  6. Article removal
  7. Logical completeness check
- Verb conjugation map with 60+ irregular verbs
- Python package via PyO3/Maturin
- Pytest test suite (53 tests)
- MIT License
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
- GitHub Actions CI workflow
- GitHub issue templates (bug report, feature request)

### Fixed
- Case-insensitive connective removal
- "this" article removal
- Trailing period stripping in active-voice agent position
- Word-merging prevention after connective elimination