use pyo3::prelude::Python;
use regex::Regex;
use std::env;

// These are defined in src/lib.rs, but we can copy them here for testing
fn transform_active_voice(_py: Python, text: &str) -> Result<String, String> {
    let pattern = Regex::new(r#"(?i)(\w+)\s+(was|were)\s+(\w+)\s+by\s+(\w+)"#).unwrap();
    let result = pattern.replace_all(text, "$4 $3 $1");
    Ok(result.to_string())
}

fn normalize_tense(_py: Python, text: &str) -> Result<String, String> {
    let pattern = Regex::new(r#"(\w+)ed\b"#).unwrap();
    let result = pattern.replace_all(text, "$1");
    Ok(result.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        return;
    }
    let text = &args[1];

    Python::with_gil(|py| {
        // Test transform_active_voice
        match transform_active_voice(py, text) {
            Ok(result) => println!("transform_active_voice('{}'): '{}'", text, result),
            Err(e) => println!("transform_active_voice('{}'): Error: {}", text, e),
        }

        // Test normalize_tense
        match normalize_tense(py, text) {
            Ok(result) => println!("normalize_tense('{}'): '{}'", text, result),
            Err(e) => println!("normalize_tense('{}'): Error: {}", text, e),
        }
    });
}
