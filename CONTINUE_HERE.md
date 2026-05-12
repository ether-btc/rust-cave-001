# rust-cave-001 — CONTINUE_HERE

## Session: 2026-05-12 09:21 AM — Fix cp312-abi3 wheel tag → cp310-abi3

### Problem
CI produces `rust_cave_001-0.1.0-cp312-abi3-linux_x86_64.whl` instead of `cp310-abi3`.
Installing on Python 3.10 fails: "not a supported wheel on this platform".

### Root Cause (confirmed)
`abi3-py312` pyo3 feature forces minimum ABI to Python 3.12 at COMPILE TIME
via `CARGO_FEATURE_ABI3_PY312` env var → pyo3-build-config's `get_abi3_version()`
returns `3.12` regardless of interpreter settings.

Attempts that failed before finding the real fix:
1. `--python-version 3.10` maturin flag → maturin doesn't recognize it
2. `-i python3.10` maturin flag → maturin still tags cp312
3. `MATURIN_PYTHON: python3.10` → same
4. `PYO3_PYTHON: python3.10` → same (build.rs doesn't control abi3 feature)
5. build.rs respects PYO3_PYTHON → same
6. `python-version = "3.10"` in pyproject.toml + `PYTHON: python3.10` env → same
   (run 25719799686 still produced cp312 — maturin config insufficient)

### Fix Applied (commit 97c3f2c)
**Cargo.toml** — change `abi3-py312` → `abi3-py310`:
```toml
pyo3 = { version = "0.24.2", features=["extension-module","abi3-py310"] }
```

The `abi3-py312` feature hardcodes abi3 version at compile time via CARGO_FEATURE env var.
Changing to `abi3-py310` allows pyo3-build-config to generate cp310-abi3 config.

### Files Modified
- `pyproject.toml` — +`python-version = "3.10"` under `[tool.maturin]`
- `.github/workflows/ci.yml` — +`PYTHON: python3.10` env var
- `Cargo.toml` — `abi3-py312` → `abi3-py310` (THE ACTUAL FIX)
- `build.rs` — respects `PYO3_PYTHON` env var (set in earlier session)

### Git
- Latest commit: `97c3f2c` — "fix: change abi3-py312 to abi3-py310 in Cargo.toml — force cp310-abi3 wheel tag"
- Pushed: yes (3e0d072..97c3f2c master -> master)
- Previous CI runs: 25716953330, 25717705809, 25717858354, 25718719728, 25719467326, 25719799686 (all FAILED)
- New CI run triggered by push: pending

### Pending Verification
- CI run must complete with `cp310-abi3` wheel
- `pip install dist/*.whl` on Python 3.10 must succeed
- `pytest tests/` must pass (10/10 tests from last session)

### If Fix Still Fails
If `cp312` persists even after `abi3-py310`:
1. Remove `abi3-*` entirely from Cargo.toml — use plain `pyo3` with no abi3 feature
   → pyo3-build-config will target the host Python version at build time
2. This forces a platform-specific wheel (no abi3 tag) which is fine for a first release

### Key Files
- `/tmp/rust-cave-001/Cargo.toml` — pyo3 dependency (NOW: abi3-py310)
- `/tmp/rust-cave-001/pyproject.toml` — maturin config
- `/tmp/rust-cave-001/.github/workflows/ci.yml` — CI workflow
- `/tmp/rust-cave-001/build.rs` — custom build script