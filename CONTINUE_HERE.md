# CONTINUE HERE — rust-cave-001

## v0.4.1 — SHIPPED

| Item | Value |
|------|-------|
| Commit | `26d759e` |
| Tag | [v0.4.1](https://github.com/ether-btc/rust-cave-001/releases/tag/v0.4.1) |
| PyPI | Wheel built: `dist/rust_cave_001-0.4.1-cp310-abi3-manylinux_2_34_aarch64.whl` — needs manual upload (no token on this machine) |
| Rust Tests | 28 PASS |
| Python Tests | 123 PASS |

### What's New in v0.4.1
- **Plural passive voice** — `"were V-ed by"` regex transforms to active
- **Past-perfect passive voice** — `"had been V-ed by"` regex transforms to active  
- **Double-article fix** — all passive patterns no longer produce `"the The X"`
- 4 Rust + 4 Python tests for the new patterns

### Next Priorities
1. PyPI upload (needs token or GitHub Actions trusted publishing)
2. crates.io publish (needs `cargo login` token)
3. Self-learning framework — adaptive strategy tuning (benchmark ceiling 48.7%)
4. Broader passive patterns: "is being V-ed", "has been V-ed", "will be V-ed"
5. CI: re-add publish job with OIDC trusted publishing (no secrets needed)

### Key Files
| File | Path |
|------|------|
| Main lib | `src/lib.rs` |
| Classifier | `src/classifier.rs` |
| Error types | `src/error.rs` |
| Verb maps | `src/verb_maps.rs` |
| Build script | `build.rs` |
| Python tests | `tests/test_rust_cave_001.py` |
| CI workflow | `.github/workflows/ci.yml` |
