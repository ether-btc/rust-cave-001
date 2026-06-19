# v0.5.0 Release - Continuation Point

## Status: IN PROGRESS (2026-06-19)

### Completed Today
- [x] Fixed version mismatch: pyproject.toml updated from 0.4.3 → 0.5.0
- [x] Deleted and recreated v0.5.0 tag (was c759f6a, now points to version-sync'd code)
- [x] Updated CI workflow (.github/workflows/ci.yml):
  - Added `validate-versions` job (checks Cargo.toml, pyproject.toml, and git tag match)
  - Migrated from deprecated `maturin publish` to `maturin build + twine upload`
  - Added post-publish verification step (installs from PyPI, verifies version)
  - Changed secret from `PYPI_PASSWORD` to `PYPI_TOKEN` (standard convention)

### Pending - REQUIRES USER ACTION
- [ ] **Configure PYPI_TOKEN secret in GitHub repo:**
  1. Go to https://github.com/ether-btc/rust-cave-001/settings/secrets/actions
  2. Click "New repository secret"
  3. Name: `PYPI_TOKEN`
  4. Value: `pypi-A...` (get from password manager)
  5. Click "Add secret"

- [ ] **Trigger release:**
  Option A: Push a new tag (CI will auto-deploy):
  ```bash
  git tag -d v0.5.0 && git push origin --delete v0.5.0
  git tag -a v0.5.0 -m "v0.5.0: Clippy cleanup + pyo3 security updates + CI fixes"
  git push origin v0.5.0
  ```
  
  Option B: Manually run the publish job:
  ```bash
  cd /home/hermes-pi/projects/rust-cave-001
  gh workflow run ci.yml --ref v0.5.0
  ```

### Remaining Tasks
- [ ] Verify CI passes (all jobs: test ✓, audit ✓, validate-versions ✓, publish ✓)
- [ ] Verify PyPI package: `pip install rust-cave-001==0.5.0`
- [ ] Aeon filing registry entry
- [ ] Wiki update with v0.5.0 release notes

## CI Workflow Changes Summary

### New: validate-versions job
Runs before publish, ensures:
- Cargo.toml version matches git tag
- pyproject.toml version matches git tag
- Fails fast if versions don't match (prevents silent version mismatch)

### Updated: publish job
- Was: `maturin publish --skip-auditwheel` (deprecated)
- Now: `maturin build --auditwheel skip` + `twine upload` (standard approach)
- Added: Post-publish verification (installs from PyPI, imports module, checks __version__)

### Secret name change
- Was: `PYPI_PASSWORD`
- Now: `PYPI_TOKEN` (conventional name for PyPI API tokens)
- Format: `pypi-xxxxxxxxxxxxxxxxxxxx`

## Release Checklist (for future releases)

Before tagging a new version:

1. **Version sync:**
   ```bash
   # Verify both files have same version
   grep '^version' Cargo.toml pyproject.toml
   ```

2. **Run full test suite:**
   ```bash
   cargo test && pytest tests/ -v
   ```

3. **Build and verify locally:**
   ```bash
   maturin build --release -o dist/
   pip install dist/*.whl --force-reinstall
   python -c "import rust_cave_001; print(rust_cave_001.__version__)"
   ```

4. **Check CI is green** on latest commit

5. **Verify PyPI secret is configured** (one-time setup)

6. **Create tag:**
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z: <brief description>"
   git push origin vX.Y.Z
   ```

7. **Monitor CI** (should run test → audit → validate-versions → publish → verify)

8. **Verify on PyPI:**
   ```bash
   pip install rust-cave-001==X.Y.Z
   python -c "import rust_cave_001; print(rust_cave_001.__version__)"
   ```

9. **Update documentation:**
   - Aeon filing registry
   - Wiki release notes
   - CHANGELOG.md (if not already updated)

## Notes

- The `validate-versions` job will catch the version mismatch that occurred in the initial v0.5.0 release attempt
- Using `twine upload` instead of `maturin publish` follows the current best practice (maturin publish is deprecated)
- Post-publish verification ensures the artifact on PyPI actually works and has correct version