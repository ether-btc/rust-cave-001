# rust-cave-001 — Session Reference (May 15, 2026)

## Resume Here

Phase 0 complete. Next: **design and build the self-learning framework.**

Quick-start references:
- Repo: `https://github.com/ether-btc/rust-cave-001` (branch: master, tag: v0.2.0)
- Commit: `8cc1e49` — "v0.2.0: implement 3 missing SPEC rules with attribution"
- CI: All green (58/58 tests, benchmark all passing)
- Issues: #3 open (Mnemosyne integration tracking)

## What Exists

- **9 compression rules** in pipeline (src/lib.rs, 648 lines)
- **Benchmark suite** (benchmarks/benchmark.py, 9 text types, LZ4, combined pipeline)
- **58 passing tests** (tests/test_rust_cave_001.py)
- **ATTRIBUTION.md** — credits wilpel/caveman-compression, JuliusBrussee/caveman, squeez
- **Wiki:** entities/rust-cave-001.md (ecosystem mapped), concepts/self-learning-caveman-framework.md

## Key Finding

Static ceiling is **48.4% avg token reduction**. Adding present tense + pronoun rules didn't push past it. Self-learning framework (strategy selection per input type) is the only path to 55-60%.

## Next Steps (priority order)

1. **Self-learning framework design & build** — strategy selector + auto-benchmark oracle. Architecture already drafted in wiki (concepts/self-learning-caveman-framework.md). Core principle: separate crate, opt-in, Type 2 decision.
2. **Publish to PyPI** — needs API token from https://pypi.org/manage/account/token/
3. **MCP server integration** — expose as MCP tool (reference: squeez architecture)
4. **Expand verb map** — 100+ → 200+ irregular verbs
5. **Close Issue #3** — auto-detect pattern in Mnemosyne (try: import instead of env var)

## Upstream Repos to Watch

| Repo | Why | Absorb When |
|------|-----|-------------|
| wilpel/caveman-compression | Direct SPEC upstream, MLM/LLM modes | Framework phase |
| JuliusBrussee/caveman (37K ⭐) | Market validation, /caveman-compress | Post-framework |
| claudioemmanuel/squeez (~2K ⭐) | MCP server architecture, Rust hooks | Framework phase |

## Commands

```bash
# Build & test
cd ~/rust-cave-001  # or /tmp/rust-cave-001
cargo build --release
maturin build --release --auditwheel skip -o dist/
pip install dist/*.whl --force-reinstall --break-system-packages
pytest tests/ -v
python3 benchmarks/benchmark.py
```
