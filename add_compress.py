#!/usr/bin/env python3
"""Add the compress function to the lib.rs."""

# Read the current lib.rs
with open('src/lib.rs', 'r') as f:
    content = f.read()

# Define the compress function
compress_function = """

/// Compress text using all Caveman Compression rules.
/// Returns a token-reduced string suitable for LLM input or binary compression.
#[pyfunction]
#[pyo3(signature = (text))]
pub fn compress(text: &str) -> PyResult<String> {
    let result = apply_caveman_rules(text);
    if result.is_empty() {
        return Err(exceptions::PyValueError::new_err(
            \"Compression produced empty output\",
        ));
    }
    Ok(result)
}

"""

# Find the location to insert: after preprocess_text function and before module registration
# Look for the line "pub fn preprocess_text" and insert the compress function after its closing brace
# Then find the #[pymodule] and ensure compress is added

# First, let's add the compress function before the module registration
pattern = r"(/// Python Module Registration\n\[pymodule\]\nfn rust_cave_001\()"

replacement = f"""{compress_function}
{pattern.replace('fn rust_cave_001', 'fn rust_cave_001', 1)}"""

# Actually, let's insert the compress function right before the module registration
# Find the line that says "/// Python Module Registration" and insert before it
pattern = r"(/// Python Module Registration\n\[pymodule\]\nfn rust_cave_001\()"
replacement = f"""{compress_function}
\\1"""

content = re.sub(pattern, replacement, content, count=1, flags=re.MULTILINE)

with open('src/lib.rs', 'w') as f:
    f.write(content)

print("Added compress function to lib.rs.")