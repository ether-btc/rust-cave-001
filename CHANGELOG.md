# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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