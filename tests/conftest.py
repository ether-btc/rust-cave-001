"""Pytest configuration for rust-cave-001 tests.

Keeps test suite isolated from the parent package's __init__.py
which imports from target/release (could conflict during pytest collection).
"""
import sys
import os

# Ensure we import from the venv site-packages, not the project root
_project_root = os.path.dirname(__file__)
_vendor_path = os.path.join(_project_root, ".venv", "lib", "python3.13", "site-packages")

# Prepend vendor path so rust_cave_001 is resolved from the installed package
if _vendor_path not in sys.path:
    sys.path.insert(0, _vendor_path)

# Set library path so the .so can be loaded
_release_path = os.path.join(_project_root, "target", "release")
os.environ["LD_LIBRARY_PATH"] = _release_path + ":" + os.environ.get("LD_LIBRARY_PATH", "")