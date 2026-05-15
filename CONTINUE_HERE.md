# rust-cave-001 — Session Reference (May 15, 2026)

## Resume Here

Phase 0 complete, framework design greenlit. **Next: build the self-learning framework.**

### Quickstart
```
git clone https://github.com/ether-btc/rust-cave-001
cd rust-cave-001
cargo build --release
maturin build --release --auditwheel skip -o dist/
pip install dist/*.whl --force-reinstall --break-system-packages
pytest tests/ -v
python3 benchmarks/benchmark.py
```

### Key References
| What | Value |
|---|---|
| Repo | `github.com/ether-btc/rust-cave-001` (master) |
| Latest commit | `71f2c6b` — "fix: normalize_present_tense ed-stripping edge cases" |
| Tags | v0.1.0, v0.1.1, v0.2.0, v0.2.1 |
| Issues | #3 OPEN (Mnemosyne integration), #2 CLOSED, #1 CLOSED |
| CI | Green — 58/58 tests, benchmark 48.4% |
| Open issue | #3: Wire Caveman into Mnemosyne (auto-detect pattern) |

### What Exists
- **9 compression rules** in pipeline (`src/lib.rs`, 676 lines) — pronoun resolution → active voice → present tense → articles → intensifiers → connectives → word limit → completeness
- **Benchmark suite** (`benchmarks/benchmark.py`) — 9 text types, LZ4, combined pipeline
- **58 passing tests** (`tests/test_rust_cave_001.py`)
- **ATTRIBUTION.md** — credits wilpel/caveman-compression, JuliusBrussee/caveman, squeez
- **Wiki pages:** `entities/rust-cave-001.md` (ecosystem mapped), `concepts/self-learning-caveman-framework.md`

### Key Finding
**Static ceiling: 48.4% avg token reduction.** Adding present tense + pronoun rules didn't push past it. Self-learning framework (strategy selection per input type) is the only path to 55-60%. Architecture already drafted in wiki.

### Next Steps (priority order)
1. **Self-learning framework design & build** — strategy selector + auto-benchmark oracle (see wiki concept page)
2. **Publish to PyPI** — needs API token from https://pypi.org/manage/account/token/
3. **MCP server integration** — expose as MCP tool (reference: squeez architecture)
4. **Expand verb map** — 100+ → 200+ irregular verbs
5. **Add direct Python tests for normalize_present_tense and resolve_pronouns**
6. **Close Issue #3** — auto-detect pattern in Mnemosyne (`try: import` instead of env var)

### Upstream Repos to Watch
| Repo | Why | Absorb When |
|------|-----|-------------|
| wilpel/caveman-compression | Direct SPEC upstream, MLM/LLM modes | Framework phase |
| JuliusBrussee/caveman (37K ⭐) | Market validation, /caveman-compress | Post-framework |
| claudioemmanuel/squeez (~2K ⭐) | MCP server architecture, Rust hooks | Framework phase |
