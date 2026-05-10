from setuptools import setup, Extension
import os

# The Rust crate name
CRATE_NAME = "rust-cave-001"

# The directory containing the compiled .so
LIB_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "target", "release")

# Define the extension module
module = Extension(
    name=CRATE_NAME,
    sources=[os.path.join(LIB_DIR, f"librust_cave_001.so")],
)

setup(
    name="rust-cave-001",
    version="0.1.0",
    ext_modules=[module],
    zip_safe=False,
)
