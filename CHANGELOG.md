# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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