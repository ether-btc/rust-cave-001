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
        ("sung", "sang"),
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

    // Regex to match passive voice: "The X was V-ed by Z" → "Z V-ed the X"
    // Pattern breakdown: "The " + (subject: one or more words) + " was " + (verb-pp) + " by " + (agent: one or more words)
    let pattern = Regex::new(r"(?i)\bThe\s+(.+?)\s+was\s+(\w+)\s+by\s+(.+)").unwrap();

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
#[allow(dead_code)]
fn normalize_tense(text: &str) -> PyResult<String> {
    // Pattern: verbs ending in "ed" -> base form (simplified)
    let pattern = Regex::new(r"(\w+)ed\b").unwrap();
    let result = pattern.replace_all(text, "$1");
    Ok(result.to_string())
}

/// Check logical completeness
fn is_logically_complete(text: &str) -> bool {
    // Simplified check: at least two words
    let pattern = Regex::new(r"\b\w+\b\s+\b\w+\b").unwrap();
    pattern.is_match(text)
}

// Split text into sentences based on punctuation (. ! ?)
fn split_into_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        current.push(c);
        
        // Check for sentence-ending punctuation followed by space or end of string
        if c == '.' || c == '!' || c == '?' {
            // Look ahead: if next char is whitespace or end, this is a sentence boundary
            match chars.peek() {
                Some(&next) if next.is_whitespace() => {
                    // End of sentence - trim and add to list
                    sentences.push(current.trim().to_string());
                    current.clear();
                    // Skip the whitespace
                    while let Some(ws) = chars.next_if(|c| c.is_whitespace()) {}
                }
                None => {
                    // End of string - add final sentence
                    sentences.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    // Not a sentence boundary (e.g., part of ellipsis "..." or abbreviation)
                    // Continue building the current sentence
                }
            }
        }
    }

    // Add any remaining text as a sentence
    if !current.trim().is_empty() {
        sentences.push(current.trim().to_string());
    }

    sentences
}

// Remove articles (the, a, an) from text
// Short sentences where removal would produce <3 words are preserved unchanged
fn remove_articles(text: &str) -> String {
    // Split into words to check length
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    
    // If sentence is too short, preserve it unchanged
    if word_count < 3 {
        return text.to_string();
    }
    
    // Pattern to match articles at word boundaries (case-insensitive)
    let pattern = Regex::new(r"(?i)\b(the|a|an)\b").unwrap();
    let result = pattern.replace_all(text, "").to_string();
    
    // Trim extra spaces left by removal
    result.trim().to_string()
}

// Remove intensifiers (very, extremely, quite, rather, really, somewhat)
// Short sentences where removal would produce <3 words are preserved unchanged
fn remove_intensifiers(text: &str) -> String {
    // Split into words to check length
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    
    // If sentence is too short, preserve it unchanged
    if word_count < 3 {
        return text.to_string();
    }
    
    // Pattern to match intensifiers at word boundaries (case-insensitive)
    let pattern = Regex::new(r"(?i)\b(very|extremely|quite|rather|really|somewhat)\b").unwrap();
    let result = pattern.replace_all(text, "").to_string();
    
    // Trim extra spaces
    result.trim().to_string()
}

// Remove connectives (because, however, therefore, but)
fn eliminate_connectives(text: &str) -> String {
    let pattern = Regex::new(r"\b(because|however|therefore|but)\b").unwrap();
    pattern.replace_all(text, "").to_string()
}

// Enforce word limit (2-5 words)
// Truncate sentences longer than 5 words by splitting on commas
fn enforce_word_limit(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    
    // If already within limit, return as is
    if word_count <= 5 {
        return text.to_string();
    }
    
    // Try to split on commas first
    if text.contains(',') {
        // Take the first clause (before the first comma)
        if let Some((first_part, _)) = text.split_once(',') {
            let first_words: Vec<&str> = first_part.split_whitespace().collect();
            if first_words.len() >= 2 && first_words.len() <= 5 {
                return first_part.trim().to_string();
            }
        }
    }
    
    // If no comma or comma split didn't give good length, take first 5 words
    let mut result_words = Vec::new();
    for word in words {
        if result_words.len() < 5 {
            result_words.push(word);
        } else {
            break;
        }
    }
    
    result_words.join(" ")
}

// Apply all Caveman compression rules in the correct order
fn apply_caveman_rules(text: &str) -> PyResult<String> {
    // 1. Split into sentences (if multiple)
    let sentences = split_into_sentences(text);
    let mut processed_sentences = Vec::new();
    
    for sentence in sentences {
        let mut result = sentence;
        
        // 2. Active voice transformation
        result = transform_active_voice(&result)?;
        
        // 3. Remove articles
        result = remove_articles(&result);
        
        // 4. Remove intensifiers
        result = remove_intensifiers(&result);
        
        // 5. Remove connectives
        result = eliminate_connectives(&result);
        
        // 6. Enforce word limit
        result = enforce_word_limit(&result);
        
        // 7. Check logical completeness (at least 2 words)
        let min_words = 2;
        let word_count = result.split_whitespace().count();
        if word_count < min_words {
            return Err(exceptions::PyValueError::new_err(
                "Text lacks logical completeness - please provide complete sentences",
            ));
        }
        
        processed_sentences.push(result);
    }
    
    // Join sentences back together
    Ok(processed_sentences.join(" "))
}

/// Apply all Caveman compression rules to the input text
#[pyfunction]
#[pyo3(signature = (text))]
pub fn compress(text: &str) -> PyResult<String> {
    apply_caveman_rules(text)
}

/// Preprocess text by applying active voice, present tense, and logical completeness checks
#[pyfunction]
#[pyo3(signature = (text))]
pub fn preprocess_text(text: &str) -> PyResult<String> {
    let mut result = String::from(text);

    // Transform to active voice (agent verb_past the subject)
    result = transform_active_voice(&result)?;

    // Active voice transformation already handles the primary goal of
    // converting passive to active voice. Present tense normalization needs a
    // proper verb conjugation database to work correctly.
    // result = normalize_tense(&result)?;

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
        compress, decompress, deserialize_compressed, estimate_tokens, get_stats, my_compress,
        preprocess_text, serialize_compressed,
    };
    module.add_function(wrap_pyfunction!(my_compress, module)?)?;
    module.add_function(wrap_pyfunction!(decompress, module)?)?;
    module.add_function(wrap_pyfunction!(estimate_tokens, module)?)?;
    module.add_function(wrap_pyfunction!(get_stats, module)?)?;
    module.add_function(wrap_pyfunction!(serialize_compressed, module)?)?;
    module.add_function(wrap_pyfunction!(deserialize_compressed, module)?)?;
    module.add_function(wrap_pyfunction!(preprocess_text, module)?)?;
    module.add_function(wrap_pyfunction!(compress, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_logically_complete() {
        assert!(is_logically_complete("Hello world"));
        assert!(!is_logically_complete(""));
        assert!(!is_logically_complete("Hello"));
    }

    #[test]
    fn test_remove_articles() {
        let result1 = remove_articles("The database needs an index");
        assert!(!result1.to_lowercase().contains("the"));
        
        let result2 = remove_articles("An apple a day");
        assert!(!result2.contains(" a "));
        assert!(!result2.contains(" A "));
        
        let result3 = remove_articles("A test");
        assert!(!result3.contains(" a "));
        assert!(!result3.contains(" A "));
    }

    #[test]
    fn test_transform_active_voice() {
        let result = transform_active_voice("The ball was thrown by John").unwrap();
        println!("Debug: result = '{}'", result);
        assert!(result.contains("John"));
        assert!(result.contains("threw"));
        assert!(!result.contains("the"));
    }
}
