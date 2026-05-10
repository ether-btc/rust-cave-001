use lz4::block::{self, CompressionMode};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

#[pyfunction]
#[pyo3(signature = (data, level = 9))]
/// Compress data using LZ4 algorithm
pub fn my_compress(data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    let mode = CompressionMode::HIGHCOMPRESSION(level);
    let compressed =
        block::compress(data, Some(mode), true).map_err(|e| exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(compressed)
}

#[pyfunction]
/// Decompress data using LZ4 algorithm
pub fn decompress(data: &[u8]) -> PyResult<Vec<u8>> {
    let decompressed =
        block::decompress(data, None).map_err(|e| exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(decompressed)
}

#[pyfunction]
/// Estimate token count using regex pattern
pub fn estimate_tokens(text: &str) -> PyResult<usize> {
    let re = Regex::new(r"\b\w+\b").map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
    let count = re.find_iter(text).count();
    Ok(count)
}

#[pyfunction]
/// Get compression statistics
pub fn get_stats(compressed: &[u8], original: &[u8]) -> PyResult<PyObject> {
    let original_size = original.len() as f64;
    let compressed_size = compressed.len() as f64;
    let ratio = original_size / compressed_size;
    let saved = original_size - compressed_size;
    let percentage = (saved / original_size) * 100.0;

    Python::with_gil(|py| {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("original_size", original_size)?;
        dict.set_item("compressed_size", compressed_size)?;
        dict.set_item("ratio", ratio)?;
        dict.set_item("saved_bytes", saved)?;
        dict.set_item("saved_percent", percentage)?;
        Ok(dict.into())
    })
}

#[pyfunction]
#[pyo3(signature = (serialized_data, level = 9))]
/// Compress already-serialized data
pub fn serialize_compressed(serialized_data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    my_compress(serialized_data, level)
}

#[pyfunction]
/// Decompress data back to serialized form
pub fn deserialize_compressed(data: &[u8]) -> PyResult<Vec<u8>> {
    decompress(data)
}

/// Convert passive voice to active voice using regex patterns
fn transform_active_voice(text: &str) -> PyResult<String> {
    // Pattern: "The X was V-ed by Z" → "Z V-ed the X"
    // Examples: "The ball was thrown by John" → "John threw the ball"
    //           "The cake was eaten by Mary" → "Mary ate the cake"

    // Map of past participles to simple past forms (irregular verbs)
    let verb_conjugations: std::collections::HashMap<&str, &str> = [
        ("thrown", "threw"),
        ("eaten", "ate"),
        ("written", "wrote"),
        ("seen", "saw"),
        ("done", "did"),
        ("given", "gave"),
        ("taken", "took"),
        ("made", "made"),
        ("found", "found"),
        ("told", "told"),
        ("called", "called"),
        ("used", "used"),
        ("asked", "asked"),
        ("wanted", "wanted"),
        ("needed", "needed"),
        ("looked", "looked"),
        ("worked", "worked"),
        ("played", "played"),
        ("moved", "moved"),
        ("lived", "lived"),
        ("believed", "believed"),
        ("happened", "happened"),
        ("changed", "changed"),
        ("showed", "showed"),
        ("watched", "watched"),
        ("followed", "followed"),
        ("stopped", "stopped"),
        ("created", "made"),
        ("brought", "brought"),
        ("heard", "heard"),
        ("held", "held"),
        ("sent", "sent"),
        ("built", "built"),
        ("understood", "understood"),
        ("drawn", "drew"),
        ("grown", "grew"),
        ("flown", "flew"),
        ("broken", "broke"),
        ("drunk", "drank"),
        ("sunk", "sank"),
        ("spun", "spun"),
        ("run", "ran"),
        ("read", "read"),
        ("cut", "cut"),
        ("put", "put"),
        ("set", "set"),
        ("shut", "shut"),
        ("cost", "cost"),
        ("hurt", "hurt"),
        ("let", "let"),
        ("regretted", "regretted"),
    ]
    .iter()
    .cloned()
    .collect();

    // Regex to match passive voice: "The X was V-ed by Z"
    // Pattern breakdown: "The " + (subject) + " was " + (verb-pp) + " by " + (agent)
    let pattern = Regex::new(r"(?i)\bThe\s+(\w+)\s+was\s+(\w+)\s+by\s+(\w+)").unwrap();

    let result = pattern.replace_all(text, |caps: &regex::Captures| {
        let subject = &caps[1];
        let verb_pp = &caps[2].to_lowercase();
        let agent = &caps[3];

        // Look up conjugated verb form
        let verb_past = verb_conjugations
            .get(verb_pp.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // Fallback: try to handle regular verbs by removing "ed"
                if verb_pp.ends_with("ed") {
                    verb_pp[..verb_pp.len() - 2].to_string()
                } else {
                    verb_pp.to_string()
                }
            });

        // Return: "agent verb_past the subject"
        format!("{} {} the {}", agent, verb_past, subject)
    });

    Ok(result.to_string())
}

/// Normalize text to present tense
fn normalize_tense(text: &str) -> PyResult<String> {
    // Pattern: verbs ending in "ed" -> base form (simplified)
    let pattern = Regex::new(r"(\w+)ed\b").unwrap();
    let result = pattern.replace_all(text, "$1");
    Ok(result.to_string())
}

/// Check logical completeness
fn is_logically_complete(text: &str) -> bool {
    // Simplified check: at least three words
    let pattern = Regex::new(r"\b\w+\b.*\b\w+\b.*\b\w+\b").unwrap();
    pattern.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_completeness() {
        // Should pass: 3+ words
        assert!(is_logically_complete("The cat was chased by the dog"));
        assert!(is_logically_complete("The dog chased the cat"));
        assert!(is_logically_complete("This is a test"));
        assert!(is_logically_complete("I am here"));

        // Should fail: less than 3 words
        assert!(!is_logically_complete("Hello world")); // 2 words
        assert!(!is_logically_complete("Hello")); // 1 word
        assert!(!is_logically_complete("")); // 0 words
    }
}

#[pyfunction]
#[pyo3(signature = (text))]
/// Preprocess text by applying active voice, present tense, and logical completeness checks
pub fn preprocess_text(text: &str) -> PyResult<String> {
    let mut result = String::from(text);

    // Transform to active voice (agent verb_past the subject)
    result = transform_active_voice(&result)?;

    // NOTE: normalize_tense is DISABLED because it incorrectly strips 'ed' from
    // conjugated verbs like "made" (from "created") → "mak", breaking output.
    // The active voice transformation already handles the primary goal of
    // converting passive to active voice. Present tense normalization needs a
    // proper verb conjugation database to work correctly.
    // result = normalize_tense(&result)?;

    // Debug: print intermediate result
    eprint!("DEBUG: after transformations: '{}'\n", result);
    std::io::Write::flush(&mut std::io::stderr()).unwrap();

    // Check logical completeness
    if !is_logically_complete(&result) {
        return Err(exceptions::PyValueError::new_err(
            "Text lacks logical completeness - please provide complete sentences",
        ));
    }

    Ok(result)
}

#[pymodule]
fn rust_cave_001(
    _py: Python,
    module: &pyo3::prelude::Bound<'_, pyo3::types::PyModule>,
) -> PyResult<()> {
    use crate::{
        decompress, deserialize_compressed, estimate_tokens, get_stats, my_compress,
        preprocess_text, serialize_compressed,
    };
    module.add_function(wrap_pyfunction!(my_compress, module)?)?;
    module.add_function(wrap_pyfunction!(decompress, module)?)?;
    module.add_function(wrap_pyfunction!(estimate_tokens, module)?)?;
    module.add_function(wrap_pyfunction!(get_stats, module)?)?;
    module.add_function(wrap_pyfunction!(serialize_compressed, module)?)?;
    module.add_function(wrap_pyfunction!(deserialize_compressed, module)?)?;
    module.add_function(wrap_pyfunction!(preprocess_text, module)?)?;
    Ok(())
}