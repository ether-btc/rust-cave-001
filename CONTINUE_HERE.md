# rust-cave-001 — CONTINUE_HERE

## Session: 2026-05-12 09:21 AM — Fix cp312-abi3 wheel tag → cp310-abi3

### Problem
CI produces `rust_cave_001-0.1.0-cp312-abi3-linux_x86_64.whl` instead of `cp310-abi3`.
Installing on Python 3.10 fails: "not a supported wheel on this platform".

### Root Cause (confirmed)
`abi3-py312` pyo3 feature forces minimum ABI to Python 3.12 at COMPILE TIME
via `CARGO_FEATURE_ABI3_PY312` env var → pyo3-build-config's `get_abi3_version()`
returns `3.12` regardless of which Python interpreter is running.
Maturin's wheel tagging reads pyo3's generated config file, which encodes cp312.

Previous attempts that failed:
- `--python-version 3.10` maturin flag → maturin doesn't recognize it
- `-i python3.10` maturin flag → maturin still tags cp312
- `MATURIN_PYTHON: python3.10` → same
- `PYO3_PYTHON: python3.10` → same (build.rs only controls sysconfig, not abi3 feature)
- build.rs respects PYO3_PYTHON → same (abi3 feature overrides build.rs)

### Fix Applied (commit 743b116)
Two changes to force maturin to target Python 3.10:

**pyproject.toml** — added `python-version = "3.10"` to `[tool.maturin]`:
```toml
[tool.maturin]
module-name = "rust_cave_001"
bindings = "pyo3"
python-version = "3.10"
```

**.github/workflows/ci.yml** — added `PYTHON: python3.10` env var:
```yaml
- name: Build and install
  env:
    PYO3_PYTHON: python3.10
    PYTHON: python3.10
  run: maturin build --release --auditwheel skip -o dist/
```

### Files Modified
- `pyproject.toml` — +1 line (`python-version = "3.10"`)
- `.github/workflows/ci.yml` — +1 line (`PYTHON: python3.10`)

### Git
- Commit: `743b116` — "fix: maturin python-version=3.10 and PYTHON env var to force cp310 abi3 wheel"
- Pushed: yes (9e4e5f7..743b116 master -> master)
- Latest CI run: 25719467326 (before this fix — FAILED cp312)
- Next CI run pending: triggered by push

### Pending Verification
- CI run needs to complete with `cp310-abi3` wheel
- `pip install dist/*.whl` on Python 3.10 must succeed
- `pytest tests/` must pass

### Key Files
- `/tmp/rust-cave-001/pyproject.toml` — maturin config
- `/tmp/rust-cave-001/.github/workflows/ci.yml` — CI workflow
- `/tmp/rust-cave-001/build.rs` — custom build script (respects PYO3_PYTHON)
- `/tmp/rust-cave-001/Cargo.toml` — `pyo3 = { version = "0.24", features = ["extension-module", "abi3-py312"] }`

### If Fix Fails Again
If CI still produces `cp312-abi3`:
1. Remove `abi3-py312` from Cargo.toml features entirely (use default `extension-module` only)
   — This lets pyo3-build-config detect the actual Python version at build time.
2. The `abi3-py312` feature is the direct cause — it hardcodes abi3 version at compile time.