#!/usr/bin/env bash
# =============================================================================
# rust-cave-001 Uninstall Script
# =============================================================================
# Removes all rust-cave-001 components from a system:
#   1. Python package (rust-cave-001 / rust_cave_001)
#   2. Hermes plugins (caveman-compression, rust_cave_output)
#   3. Config sections (caveman:, rust_cave:) from ~/.hermes/config.yaml
#   4. Optional: Hermes-agent integration (PR #24174 — only if merged)
#
# Usage:
#   ./scripts/uninstall.sh [--dry-run] [--verbose]
#   ./scripts/uninstall.sh --hermes-only   # skip Python package
#   ./scripts/uninstall.sh --python-only   # skip Hermes plugins + config
#
# Exit codes:
#   0  — success (or nothing to do)
#   1  — error
# =============================================================================

set -euo pipefail

DRY_RUN=false
VERBOSE=false
HERMES_ONLY=false
PYTHON_ONLY=false

HERMES_DIR="${HOME}/.hermes"
CONFIG_FILE="${HERMES_DIR}/config.yaml"
PLUGIN_DIR="${HERMES_DIR}/plugins"
PYTHON_PKG="rust-cave-001"

# ── Colours ─────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${BLUE}[info]${RESET}  $*"; }
success() { echo -e "${GREEN}[done]${RESET}  $*"; }
warn()    { echo -e "${YELLOW}[warn]${RESET}  $*"; }
error()   { echo -e "${RED}[error]${RESET} $*" >&2; }
dry()     { [[ "$DRY_RUN" == true ]] && echo -e "${YELLOW}[dry]${RESET}  $*" || true; }
verb()    { [[ "$VERBOSE" == true ]] || return 0; echo "  $*"; }

# ── Helpers ───────────────────────────────────────────────────────────────────
run() {
    [[ "$VERBOSE" == true ]] && echo "  Running: $*"
    [[ "$DRY_RUN" == true ]] && return 0
    eval "$@"
}

has_command() { command -v "$1" >/dev/null 2>&1; }

# ── YAML section removal (Python state-machine, no pyyaml needed) ─────────────
#
# Removes a top-level "key:\n  nested..." block from YAML.
# Handles any indentation depth for nested content.
#
_remove_yaml_section_py() {
    local section="$1"
    local config="$2"
    local backup="${config}.backup~"
    local tmp="${config}.tmp"

    if [[ "$DRY_RUN" == true ]]; then
        dry "python3 -c '...' to remove '${section}:' section from ${config}"
        return 0
    fi

    # Backup before mutation
    cp "$config" "$backup"

    # Python state-machine: skip lines while inside the target section
    local pycode="
import sys

section = ${section@Q}
skipping = False
for line in sys.stdin:
    stripped = line.rstrip()
    # Enter section: unindented line matching 'section:'
    if not skipping and stripped == section + ':':
        skipping = True
        continue
    # Exit section: non-blank, non-indented line (new top-level key)
    if skipping and stripped and not stripped.startswith(' ') and not stripped.startswith('\t'):
        skipping = False
    if not skipping:
        sys.stdout.write(line)
"
    if ! python3 -c "$pycode" < "$config" > "$tmp"; then
        error "Python YAML parser failed for section '${section}'"
        rm -f "$tmp"
        rm -f "$backup"
        return 1
    fi

    if ! mv "$tmp" "$config"; then
        error "Failed to replace config file"
        rm -f "$tmp"
        rm -f "$backup"
        return 1
    fi

    rm -f "$backup"
    return 0
}

# ── Parse args ────────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)    DRY_RUN=true ;;
        --verbose|-v) VERBOSE=true ;;
        --hermes-only)   HERMES_ONLY=true ;;
        --python-only)   PYTHON_ONLY=true ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo "  --dry-run        preview changes without applying them"
            echo "  --verbose,-v     print each command before running"
            echo "  --hermes-only    only remove Hermes plugins and config"
            echo "  --python-only    only remove Python package"
            echo "  --help,-h        show this message"
            exit 0
            ;;
        *) error "Unknown option: $1"; exit 1 ;;
    esac
    shift
done

# Validate exclusive-ish options
if [[ "$HERMES_ONLY" == true && "$PYTHON_ONLY" == true ]]; then
    error "--hermes-only and --python-only cannot be used together"
    exit 1
fi

# ── Banner ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}rust-cave-001 Uninstall${RESET}"
echo "================================"
[[ "$DRY_RUN" == true ]] && echo -e "${YELLOW}[dry-run mode — no changes will be made]${RESET}"
echo ""

# ── 1. Python package ────────────────────────────────────────────────────────
if [[ "$HERMES_ONLY" == false ]]; then
    echo -e "${BOLD}Step 1/3 — Python package${RESET}"

    if pip show "$PYTHON_PKG" >/dev/null 2>&1; then
        info "Found installed: $PYTHON_PKG"
        if [[ "$DRY_RUN" == true ]]; then
            dry "pip uninstall ${PYTHON_PKG} -y"
        else
            run "pip uninstall ${PYTHON_PKG} -y" && success "Python package removed" \
                || warn "pip uninstall failed — remove manually with: pip uninstall ${PYTHON_PKG}"
        fi
    else
        info "Python package not installed — skipping"
    fi
fi

# ── 2. Hermes plugins ────────────────────────────────────────────────────────
if [[ "$PYTHON_ONLY" == false ]]; then
    echo ""
    echo -e "${BOLD}Step 2/3 — Hermes plugins${RESET}"

    for plugin in caveman-compression rust_cave_output; do
        plugin_path="${PLUGIN_DIR}/${plugin}"

        # Remove plugin directory
        if [[ -d "$plugin_path" ]]; then
            info "Removing: ${plugin_path}"
            if [[ "$DRY_RUN" == true ]]; then
                dry "rm -rf '${plugin_path}'"
            else
                run "rm -rf '${plugin_path}'" && success "Removed plugin directory: $plugin"
            fi
        else
            info "Plugin directory not found: $plugin — skipping"
        fi

        # Remove from plugins.enabled list in config
        if [[ -f "$CONFIG_FILE" ]]; then
            if grep -qE "^[[:space:]]+-[[:space:]]+${plugin}$" "$CONFIG_FILE" 2>/dev/null; then
                info "Removing '${plugin}' from plugins.enabled"
                if [[ "$DRY_RUN" == true ]]; then
                    dry "sed -i '/^[[:space:]]*-[[:space:]]*${plugin}$/d' '${CONFIG_FILE}'"
                else
                    run "sed -i '/^[[:space:]]*-[[:space:]]*${plugin}$/d' '${CONFIG_FILE}'"
                fi
            fi
        fi
    done
fi

# ── 3. Config sections ───────────────────────────────────────────────────────
if [[ "$PYTHON_ONLY" == false ]]; then
    echo ""
    echo -e "${BOLD}Step 3/3 — Config sections${RESET}"

    if [[ ! -f "$CONFIG_FILE" ]]; then
        info "No config file at ${CONFIG_FILE} — skipping config cleanup"
    else
        for section in caveman rust_cave; do
            if grep -qE "^${section}:" "$CONFIG_FILE" 2>/dev/null; then
                info "Removing '${section}:' section from config.yaml"
                verb "Running Python YAML state-machine on ${CONFIG_FILE}"
                rc=0
                _remove_yaml_section_py "$section" "$CONFIG_FILE" || rc=$?
                if [[ "$DRY_RUN" == false && $rc -eq 0 ]]; then
                    success "Removed config section: $section"
                fi
            else
                info "Config section '${section}:' not found — skipping"
            fi
        done
    fi
fi

# ── Summary ──────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}Uninstall complete.${RESET}"
echo ""
echo "What was removed (unless --dry-run):"
[[ "$HERMES_ONLY" == false ]] && echo "  • Python package: $PYTHON_PKG"
[[ "$PYTHON_ONLY" == false ]] && echo "  • Hermes plugins: caveman-compression, rust_cave_output"
[[ "$PYTHON_ONLY" == false ]] && echo "  • Config sections: caveman:, rust_cave:"
echo ""
echo "Manual steps (if applicable):"
echo "  • If PR #24174 was merged into NousResearch/hermes-agent,"
echo "    remove tools/caveman_compression.py and toolset registration"
echo "  • Reload hermes-agent: hermes restart (or restart the service)"
echo ""
