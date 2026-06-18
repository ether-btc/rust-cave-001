# CONTINUE HERE — rust-cave-001

> **Last updated:** 2026-06-18
> **Local HEAD:** master @ `10d960d` (v0.4.3 base)
> **Active branch:** `v0.5.0-clippy-pedantic` @ `0e41988` (PR #13 open)
> **Test count:** 38 Rust + 129 Python (all green, fresh wheel)

---

## Where We Are (2026-06-18)

The 2026-06-05 CONTINUE_HERE was significantly stale. Here's the actual state:

### Released

| Version | Date | Notes |
|---------|------|-------|
| v0.4.0 | 2026-05-18 | Initial 9-rule pipeline + `OnceLock` verb-map caching (PR #6) |
| v0.4.1 | 2026-05-18 | Plural (`were`) + past-perfect (`had been`) passive voice. BUG-2 pronouns. SEC-1 decompress size limits. |
| v0.4.2 | 2026-05-18 | BUG-1 acronym guard (THE, ANDROID, XOR preserved). |
| v0.4.3 | 2026-06-05 | PyPI re-upload of v0.4.2; 3-cycle audit merged via PRs #10/#11/#12. |

### Merged PRs (since v0.4.3 release)

| PR | Title | What it fixed |
|---|---|---|
| #9 | 3-cycle audit completion documentation | 2026-05-29 audit report |
| #10 | audit: 3-cycle code audit | I-2, I-3, I-4, I-6, I-7, I-8 fixed; I-1 false-positive reverted |
| #11 | fix(data): PP→SP "shrunk" → "shrank" | Data-level data fix in `verb_maps.rs:182` and `:455` |
| #12 | chore: gitignore uv.lock | Library lockfile should not be committed |
| #13 | style(clippy): drive pedantic warning count to zero (L-1) | **OPEN** — current branch |

### v0.5.0 Backlog — Status

| ID | Item | Status | Notes |
|---|---|---|---|
| L-1 | 88 clippy pedantic warnings | ✅ **DONE** (PR #13) | Was 110/81, now 0 |
| L-2 | README says "7 rules" → "11 rules" | ✅ Non-issue | README never claimed "7 rules"; grep returned 0 matches |
| L-4 | Missing connectives `then`, `thus` | ✅ **DONE** (370cda8) | Tests pass on fresh wheel |
| L-6 | Add `ruff` to CI | ✅ **DONE** (370cda8) | `.github/workflows/ci.yml` runs `ruff check .` |

### Test Status (post-L-1)

```
cargo test --lib                            → 38 passed
pytest tests/                               → 129 passed
cargo clippy --all-targets -- -D warnings   → No issues found
cargo clippy --all-targets -- -W pedantic   → 0 warnings
cargo fmt --all -- --check                  → clean
```

### Stale-Wheel Footgun (memory)

`pip install --force-reinstall dist/*.whl` does **not** always replace the
installed `.so` (the wheel's `.so` MD5 differs from the installed `.so`
MD5 after a "successful" reinstall). When a test fails unexpectedly on
behaviour that the source already covers, verify with
`md5sum dist/*.whl …/site-packages/…/rust_cave_001.abi3.so`. If they
differ, force-replace:

```bash
unzip -p dist/rust_cave_001-*-linux_aarch64.whl \
  rust_cave_001/rust_cave_001.abi3.so \
  > …/site-packages/rust_cave_001/rust_cave_001.abi3.so
```

Hit this on 2026-06-18 — L-4 (then/thus) tests appeared to fail, but
the code was correct; the `.so` was from June 6.

---

## Next Steps (in order)

### 1. Multi-cycle audit on the L-1 delta (PR #13)

Before merging, run the standard 3-cycle audit (ponytail → correctness →
integration) on the `0e41988` commit. The L-1 changes are mostly
formatting and doc additions, but:
- `recommended_strategy()` had a refactor (extracted `FULL_PIPELINE` +
  merged Technical/Mixed arms) — needs verification that the returned
  slices are still valid and that `classify_text()` callers see no
  behaviour change.
- The module-level `#[allow(cast_*)]` in `classifier.rs` covers the
  `as f64` / `as usize` casts in `classify()` and the helper
  functions — verify no silent precision loss surfaces in the
  classifier tests.

### 2. v0.5.0 release

Once PR #13 merges:

1. Bump version in `Cargo.toml` and `pyproject.toml` (or wherever the
   version lives — check `maturin` build config).
2. `cargo build --release` + `maturin build --release --auditwheel skip`.
3. Smoke-test in a fresh venv.
4. `git tag -a v0.5.0` + push tag.
5. PyPI upload: the existing CI workflow already triggers on tag push
   (see `.github/workflows/ci.yml` PyPI publish job). Verify it fires.
6. GitHub release: `gh release create v0.5.0` with release notes
   derived from CHANGELOG.

### 3. crates.io publish (deferred — needs token)

The `CONTINUE_HERE` from 2026-06-05 noted "crates.io publish (needs
token)". No `CRATES_IO_TOKEN` is in the env. If the user wants this,
add the token to `~/.cargo/config.toml` or use `cargo login`.

### 4. Strategic direction: self-learning framework

Per `~/wiki/concepts/self-learning-caveman-framework.md`. The
`classifier.rs` module is the first component (text-type detection +
strategy selection). Next pieces:
- Benchmark oracle: run compress() on a labelled corpus, log input
  → output → effectiveness metrics
- EffectivenessTracker: per-rule stats (firing count, compression
  ratio, semantic-preservation score)
- Adaptive policy: auto-tune strategy selection based on oracle data

This is a multi-session, multi-PR project. Pick it up when the
infrastructure backlog is clear.

---

## Key Files

| File | Path | Notes |
|------|------|-------|
| Main lib | `src/lib.rs` (~52 KB now) | 9-rule pipeline + LZ4 |
| Classifier | `src/classifier.rs` (~14 KB) | Text-type heuristic + strategy selector |
| Error types | `src/error.rs` | `CompressionError` enum + `into_pyerr` |
| Verb maps | `src/verb_maps.rs` (~27 KB) | 306 PP + 357 SP→PRES (v0.4.3 base) |
| Build script | `build.rs` | Dynamic `python3-config` detection |
| Python tests | `tests/test_rust_cave_001.py` | 129 tests |
| CI workflow | `.github/workflows/ci.yml` | cargo + pytest + ruff |
| CHANGELOG | `CHANGELOG.md` | Update for v0.5.0 entry |
| AUDIT | `AUDIT.md` | 3-cycle audit log (last update 2026-06-05) |

---

## Next Session Commands

```bash
# Pickup marker: "Resume rust-cave-001 v0.5.0 work"
cd ~/projects/rust-cave-001
git checkout v0.5.0-clippy-pedantic

# Verify state
git status
git log --oneline -5
gh pr view 13

# Run the 3-cycle audit on the L-1 delta
# (use the multi-cycle-code-audit skill — load it from ~/.hermes/skills)

# If audit is clean, merge
gh pr merge 13 --squash
git checkout master
git pull
```

---

## Reference Links

- **Repo:** https://github.com/ether-btc/rust-cave-001
- **PR #13:** https://github.com/ether-btc/rust-cave-001/pull/13
- **PyPI:** https://pypi.org/project/rust-cave-001/
- **Wiki entity:** ~/wiki/entities/rust-cave-001.md
- **Self-learning framework concept:** ~/wiki/concepts/self-learning-caveman-framework.md
- **Multi-cycle audit skill:** `~/.hermes/skills/devops/multi-cycle-code-audit/`
