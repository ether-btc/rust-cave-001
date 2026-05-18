# Uninstalling rust-cave-001

This document covers complete removal of every rust-cave-001 component from a system.

## Components

| Component | Location | Installed by |
|-----------|----------|--------------|
| Python package | `site-packages/rust_cave_001/` | `maturin develop`, `pip install` |
| Hermes plugin | `~/.hermes/plugins/caveman-compression/` | manual |
| Hermes plugin | `~/.hermes/plugins/rust_cave_output/` | manual |
| Config | `~/.hermes/config.yaml` (`caveman:`, `rust_cave:`) | plugin activation |
| Hermes tools | `hermes-agent/tools/caveman_compression.py` | PR #24174 (if merged) |

---

## Automated uninstall (recommended)

From the repository root:

```bash
./scripts/uninstall.sh          # full uninstall
./scripts/uninstall.sh --dry-run  # preview only
./scripts/uninstall.sh --hermes-only  # skip Python package
./scripts/uninstall.sh --python-only  # skip Hermes plugins
```

Then reload Hermes:

```bash
hermes restart   # or restart the hermes-agent service
```

---

## Manual uninstall

### 1. Python package

```bash
# Find where it's installed
pip show rust-cave-001

# Remove
pip uninstall rust-cave-001 -y

# Or if installed editable from source
pip uninstall rust_cave_001 -y
```

### 2. Hermes plugins

Remove plugin directories:

```bash
rm -rf ~/.hermes/plugins/caveman-compression
rm -rf ~/.hermes/plugins/rust_cave_output
```

Remove from `plugins.enabled` in `~/.hermes/config.yaml`:

```yaml
plugins:
  enabled:
    - mnemosyne      # keep these
    - planforge
    # - caveman-compression   # remove this line
    # - rust_cave_output      # remove this line
    - hermes-lcm
    - rtk-rewrite
```

Remove config sections from `~/.hermes/config.yaml`:

```yaml
# Remove this section (if present):
rust_cave:
  enabled: true
  timeout_ms: 5000
  min_words: 10
  min_reduction_pct: 10
```

Note: The `caveman:` section may be present depending on your setup; it follows the same removal pattern as `rust_cave:` above.

### 3. Hermes-agent tools (PR #24174 — only if merged)

If PR #24174 was merged into `NousResearch/hermes-agent`:

```bash
# Remove the tool file
rm -f /path/to/hermes-agent/tools/caveman_compression.py

# Remove from toolsets registration in toolsets.py
# Look for: "caveman_compression" in toolsets.py and remove it
```

---

## After uninstallation

1. **Reload hermes-agent** so the plugin is fully deregistered
2. **Verify removal**:
   ```bash
   hermes tools list | grep caveman    # should be empty
   python -c "import rust_cave_001"    # should raise ImportError
   ```

---

## Troubleshooting

**Plugin still appears after removal**

The plugin may be cached. Run `hermes restart` and confirm the plugin directory is gone from `~/.hermes/plugins/`.

**Config section reappears**

Another plugin may be writing it. Check for any startup scripts that regenerate config.

**pip uninstall says package not found but import still works**

You may have multiple Python environments. Check which Python is being used:

```bash
which python
pip show rust-cave-001
```

Then uninstall from the correct environment.
