# Attribution

## Upstream Sources

### Caveman Compression SPEC & Original Implementation

- **Repository:** [wilpel/caveman-compression](https://github.com/wilpel/caveman-compression)
- **License:** MIT
- **Role:** Defines the Caveman Compression specification (SPEC.md) that rust-cave-001 implements. The 9 compression rules (sentence atomicity, word limit, connective elimination, active voice & present tense, preserve specifics, intensifier removal, article omission, pronoun handling, logical completeness) originate from this specification.
- **Files derived from SPEC:**
  - `src/lib.rs` — all 7 compression rules (adapted from SPEC rules 1-9)
  - `README.md` — "The 7 Compression Rules" section
  - `BENCHMARKS.md` — methodology inspired by wilpel's benchmarks

### Token Reduction Concept

- **Repository:** [JuliusBrussee/caveman](https://github.com/JuliusBrussee/caveman)
- **License:** MIT
- **Role:** Market validation for output-side token compression (~37K GitHub stars). Proves massive demand for token reduction in AI agent contexts. rust-cave-001 addresses the complementary input-side problem.

### Hook-Based Compression Architecture (Reference)

- **Repository:** [claudioemmanuel/squeez](https://github.com/claudioemmanuel/squeez)
- **License:** MIT
- **Role:** Reference for Rust-based MCP server architecture and cross-host hook integration patterns.

## Third-Party Rust Crates

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| [pyo3](https://crates.io/crates/pyo3) | 0.24.2 | Apache-2.0 | Python bindings |
| [lz4](https://crates.io/crates/lz4) | 1.0 | MIT | LZ4 block compression |
| [regex](https://crates.io/crates/regex) | 1.10 | MIT/Apache-2.0 | Text pattern matching |

## Build Tools

- [Maturin](https://github.com/PyO3/maturin) — Python wheel builder for Rust projects
- [PyO3](https://github.com/PyO3/pyo3) — Rust↔Python interoperability framework

---

*This project is MIT-licensed. All upstream works referenced above are also MIT-licensed where noted. Attribution is provided as a courtesy and in the spirit of open source collaboration.*
