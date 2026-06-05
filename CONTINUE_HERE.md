# CONTINUE HERE — rust-cave-001

## v0.4.3 — RELEASED (2026-06-05)

| Item | Value |
|------|-------|
| HEAD | `45f7486` (docs: add v0.4.3 to CHANGELOG) |
| Tag | [v0.4.3](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.3) |
| PyPI | `pip install rust-cave-001==0.4.3` (already on PyPI from prior upload) |
| GitHub release | https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.3 |
| Rust tests | 28 PASS ✅ |
| Python tests | 127 PASS ✅ |
| Wheel | `rust_cave_001-0.4.3-cp310-abi3-manylinux_2_34_aarch64.whl` (900 KB) |
| Token reduction | 50% on reference text (10 → 5 tokens) |

## What Was Done This Session (2026-06-05)

1. **Synced local master with origin/master** — local was 1 commit behind
2. **Built v0.4.3 wheel** with `maturin build --release -o dist/`
3. **Smoke-tested locally** — 127/127 Python tests pass in fresh venv
4. **Discovered v0.4.3 was already on PyPI** (uploaded by CI/another process)
5. **Added CHANGELOG entry** for v0.4.3 in commit `45f7486`
6. **Created annotated tag** `v0.4.3` and pushed to origin
7. **Created GitHub release** with full release notes
8. **Updated wiki** to reflect v0.4.3 state
9. **Filed aeon entry** for the release

## Audit Status (3-cycle audit, 2026-05-29)

| ID | Severity | Status | Notes |
|----|----------|--------|-------|
| BUG-1 | HIGH | ✅ FIXED | `(?i)` on removal regexes — acronym guard added |
| BUG-2 | MEDIUM | ✅ FIXED | resolve_pronouns now replaces ALL occurrences |
| BUG-3 | MEDIUM | ✅ FIXED | Passive voice agent regex double-check for "The " prefix |
| PERF-1 | MEDIUM | ✅ FIXED | count_pattern cached via OnceLock |
| SEC-1 | LOW | ✅ FIXED | decompress() validates size limits |

## v0.5.0 Backlog (4 active items)

| ID | Item | Effort | Notes |
|----|------|--------|-------|
| **L-1** | 88 clippy pedantic warnings | Medium | Deferred from audit |
| **L-2** | README says "7 rules" — should say 11 | Small | Easy docs fix |
| **L-4** | Missing connectives `then`, `thus` | Small | Add to regex + tests |
| **L-6** | Add `ruff` to CI | Small | Quality infra |

Plus deferred: I-1/I-2 (upstream SPEC rules 5/9 gaps), crates.io publish (needs token).

## Key Files

| File | Path |
|------|------|
| Main lib | `src/lib.rs` (43 KB) |
| Classifier | `src/classifier.rs` (14 KB) |
| Error types | `src/error.rs` |
| Verb maps | `src/verb_maps.rs` (23 KB) |
| Build script | `build.rs` |
| Python tests | `tests/test_rust_cave_001.py` |
| CI workflow | `.github/workflows/ci.yml` |
| CHANGELOG | `CHANGELOG.md` |

## Next Session

The natural next direction is **v0.5.0 backlog work** — L-2, L-4, and L-6 are
all small and self-contained, can be done in one sitting. After those, the
strategic direction is the **self-learning framework** per
`~/wiki/concepts/self-learning-caveman-framework.md`.

## Verification Commands

```bash
# Build wheel
cd ~/projects/rust-cave-001
export PATH="$HOME/.cargo/bin:$PATH"
maturin build --release -o dist/ --interpreter .venv/bin/python

# Install + test
uv venv /tmp/rc-test && source /tmp/rc-test/bin/activate
uv pip install /tmp/rc-test/bin/python dist/rust_cave_001-*.whl pytest
/tmp/rc-test/bin/python -m pytest tests/ -q

# Tag + push
git tag -a vX.Y.Z -m "..." HEAD
git push origin vX.Y.Z
gh release create vX.Y.Z --repo ether-btc/rust-cave-001 --title "..." --notes "..."
```

## Reference Links

- **Repo:** https://github.com/ether-btc/rust-cave-001
- **PyPI:** https://pypi.org/project/rust-cave-001/
- **Wiki entity:** ~/wiki/entities/rust-cave-001.md
- **Self-learning framework concept:** ~/wiki/concepts/self-learning-caveman-framework.md
- **Aeon entry:** `ether-btc/rust-cave-001#v043` (filed 2026-06-05)

## CI History (most recent)

| Run | SHA | Result | Notes |
|-----|-----|--------|-------|
| 26050355755 | f0b9d5e | ✅ | docs: log BUG-2 and SEC-1 |
| 26050043345 | 00f2202 | ✅ | BUG-2 + SEC-1 fixes |
| 26049619756 | be8f3d6 | ✅ | docs: update CONTINUE_HERE |
