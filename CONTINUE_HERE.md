# RUST-CAVE-001 Public Readiness Audit — CONTINUE HERE

## Last Status: 2026-05-12
**Verdict: Ready for public.** Repo is already public (MIT license), green CI, 58/58 tests pass, complete docs.

## Audit Findings (Non-blocking Polish Items)

### To Fix Next Session
1. **`pyproject.toml` missing PyPI metadata** — No `description`, `authors`, `requires-python`, `readme`, or `classifiers`. Required for PyPI publishing and crate registration.
2. **`normalize_tense` dead code** (`src/lib.rs:231-236`) — Has `#[allow(dead_code)]` stub. Either implement, remove, or mark with `TODO`.
3. **Duplicate verb entries** in `transform_active_voice` HashMap — `broken/broke` (lines 112, 128), `drawn/drew` (109, 132), `drunk/drank` (114, 133), plus several others. HashMap silently overwrites so no functional bug, but wastes code and confuses readers.
4. **`cargo test` fails** — Expected for PyO3 `cdylib` (needs `lib` crate-type for `cargo test`). CONTRIBUTING.md should note "tests run via `pytest`, not `cargo test`".
5. **`conftest.py` fragile venv path** — Assumes `.venv/lib/python*/site-packages` glob; breaks for `uv`/`pixi`/`nix`.

### Current Test Results
- `cargo fmt --check`: PASS
- `cargo clippy -- -D warnings`: PASS
- `maturin develop`: PASS
- `pytest tests/ -v`: 58/58 PASS
- `cargo test`: FAILS (expected PyO3 cdylib limitation)

### CI Status
- Latest run: GREEN (conclusion: success, branch: master)
- All workflow steps: fmt, clippy, maturin build, pytest

### Repo State
- Remote: https://github.com/ether-btc/rust-cave-001
- Branch: master, latest push: 2026-05-12T11:55:40Z
- Local clone: ~/rust-cave-001
