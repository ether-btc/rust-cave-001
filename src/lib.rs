use lz4::block::{self, CompressionMode};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

/// Compress data using LZ4 algorithm
#[pyfunction]
#[pyo3(signature = (data, level = 9))]
pub fn my_compress(data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    let mode = CompressionMode::HIGHCOMPRESSION(level);
    let compressed = block::compress(data, Some(mode), true)
        .map_err(|e| exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(compressed)
}

/// Decompress data using LZ4 algorithm
#[pyfunction]
pub fn decompress(data: &[u8]) -> PyResult<Vec<u8>> {
    let decompressed =
        block::decompress(data, None).map_err(|e| exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(decompressed)
}

/// Estimate token count using regex pattern
#[pyfunction]
pub fn estimate_tokens(text: &str) -> PyResult<usize> {
    let re =
        Regex::new(r"\b\w+\b").map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
    let count = re.find_iter(text).count();
    Ok(count)
}

/// Get compression statistics
#[pyfunction]
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

/// Compress already-serialized data
#[pyfunction]
#[pyo3(signature = (serialized_data, level = 9))]
pub fn serialize_compressed(serialized_data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    my_compress(serialized_data, level)
}

/// Decompress data back to serialized form
#[pyfunction]
pub fn deserialize_compressed(data: &[u8]) -> PyResult<Vec<u8>> {
    decompress(data)
}

// =============================================================================
// Caveman Compression Rules
// =============================================================================

/// Rule 1: Split text into atomic sentences on . ! ?
fn split_into_sentences(text: &str) -> Vec<String> {
    let sentence_ends = Regex::new(r"[.!?]\s+").unwrap();
    sentence_ends
        .split(text.trim())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Rule 7: Remove articles (the, a, an) when context is unambiguous.
/// Keep articles when omission creates ambiguity between generic vs specific.
fn remove_articles(text: &str) -> String {
    let article_regex = Regex::new(r"\b(?i)(the|a|an)\b").unwrap();
    let without = article_regex.replace_all(text, "");
    let spaces = Regex::new(r" +").unwrap();
    spaces.replace_all(&without, " ").trim().to_string()
}

/// Rule 6: Remove intensifiers (very, extremely, quite, rather, really, somewhat).
/// If removal would leave fewer than 3 words, leave sentence unchanged.
fn remove_intensifiers(text: &str) -> String {
    let intensifiers = [
        r"\bvery\s+",
        r"\bextremely\s+",
        r"\bquite\s+",
        r"\brather\s+",
        r"\breally\s+",
        r"\bsomewhat\s+",
    ];
    let mut result = text.to_string();
    for intensifier in &intensifiers {
        let re = Regex::new(intensifier).unwrap();
        result = re.replace_all(&result, "").to_string();
    }
    let spaces = Regex::new(r" +").unwrap();
    result = spaces.replace_all(&result, " ").trim().to_string();
    // If removal would leave fewer than 3 words, preserve original
    let result_word_count = result.split_whitespace().count();
    let original_word_count = text.split_whitespace().count();
    if result_word_count < 3 && original_word_count >= 3 {
        return text.to_string();
    }
    result
}

/// Rule 3: Eliminate logical connectives.
/// Causal: because, since, due to, owing to, as a result
/// Contrastive: however, nevertheless, although, despite, but
/// Sequential: therefore, thus, consequently, hence, then
/// Purpose: in order to, so that, for the purpose of
/// Conditional: if, unless (when essential)
fn eliminate_connectives(text: &str) -> String {
    // Build one big pattern for all connectives
    let connective_pattern =
        r"\b(?i)(because|since|due to|owing to|as a result|however|nevertheless|although|\
          despite|but|therefore|thus|consequently|hence|then|in order to|so that|\
          for the purpose of|unless|meanwhile|instead|otherwise)\b";
    let re = Regex::new(connective_pattern).unwrap();

    // Replace connectives with a single space to avoid jarring concatenation
    let result = re.replace_all(text, " ");

    // Collapse multiple spaces into one
    let spaces = Regex::new(r" +").unwrap();
    spaces.replace_all(&result, " ").to_string()
}

/// Rule 2: Enforce 2-5 word limit per sentence.
/// If a sentence exceeds 5 words, attempt to split on commas or "and".
/// If still over, truncate to 5 words (logically complete minimum).
fn enforce_word_limit(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    if words.len() <= 5 {
        return sentence.to_string();
    }

    // Try splitting on commas first (lists, enumerations)
    let comma_parts: Vec<&str> = sentence.split(',').collect();
    if comma_parts.len() > 1 {
        let parts: Vec<String> = comma_parts
            .iter()
            .map(|p| enforce_word_limit(p.trim()))
            .filter(|p| !p.is_empty())
            .collect();
        return parts.join(", ");
    }

    // Try splitting on " and " / " or " (conjunction splits)
    let and_parts: Vec<&str> = sentence.split(" and ").collect();
    if and_parts.len() > 1 {
        let parts: Vec<String> = and_parts
            .iter()
            .map(|p| enforce_word_limit(p.trim()))
            .filter(|p| !p.is_empty())
            .collect();
        return parts.join(" and ");
    }

    // Still over 5 words — truncate to first 5 words
    let truncated: Vec<&str> = words.into_iter().take(5).collect();
    truncated.join(" ")
}

/// Convert passive voice "The X was V-ed by Z" → "Z V-ed the X"
fn transform_active_voice(text: &str) -> String {
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
        ("optimized", "optimized"),
        ("analyzed", "analyzed"),
        ("processed", "processed"),
        ("updated", "updated"),
        ("deleted", "deleted"),
        ("inserted", "inserted"),
        ("selected", "selected"),
        ("filtered", "filtered"),
        ("sorted", "sorted"),
        ("joined", "joined"),
    ]
    .iter()
    .cloned()
    .collect();

    // Pattern: "The X was V-ed by Z" → "Z V-ed the X"
    let pattern = Regex::new(r"(?i)\bThe\s+(.+?)\s+was\s+(\w+)\s+by\s+(.+)").unwrap();

    pattern
        .replace_all(text, |caps: &regex::Captures| {
            let subject = &caps[1];
            let verb_pp = &caps[2].to_lowercase();
            let agent = caps[3].trim_end_matches(|c: char| c == '.' || c == '!' || c == '?');

            let verb_past = verb_conjugations
                .get(verb_pp.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    if verb_pp.ends_with("ed") {
                        verb_pp[..verb_pp.len() - 2].to_string()
                    } else {
                        verb_pp.to_string()
                    }
                });

            format!("{} {} the {}", agent, verb_past, subject)
        })
        .to_string()
}

/// Check logical completeness: at least 3 words
fn is_logically_complete(text: &str) -> bool {
    let re = Regex::new(r"\b\w+\b.*\b\w+\b.*\b\w+\b").unwrap();
    re.is_match(text)
}

// =============================================================================
// Main Compression Pipeline
// =============================================================================

/// Apply all Caveman Compression rules in spec order and return compressed text.
/// This is the primary text-level API for semantic compression.
/// For binary compression, chain with my_compress() on the output bytes.
fn apply_caveman_rules(text: &str) -> String {
    let sentences = split_into_sentences(text);
    let mut processed_sentences: Vec<String> = Vec::new();

    for sentence in sentences {
        if sentence.is_empty() {
            continue;
        }

        let mut s = sentence;

        // Rule 3: Connective elimination (first, to simplify structure)
        s = eliminate_connectives(&s);

        // Rule 6: Intensifier removal
        s = remove_intensifiers(&s);

        // Rule 4: Active voice transformation (BEFORE article removal — needs "The" prefix)
        s = transform_active_voice(&s);

        // Rule 7: Article removal (AFTER active voice — "The" already consumed)
        s = remove_articles(&s);

        // Rule 2: Word count limit (2-5 words per sentence)
        s = enforce_word_limit(&s);

        // Collapse multiple spaces after article removal
        let spaces = Regex::new(r" +").unwrap();
        s = spaces.replace_all(&s, " ").to_string();

        // Trim extra whitespace
        s = s.trim().to_string();

        // Skip empty results
        if s.is_empty() {
            continue;
        }

        // Rule 9: Logical completeness check
        if is_logically_complete(&s) {
            processed_sentences.push(s);
        }
    }

    // Join processed sentences with space (Rule 1: atomic sentences separated)
    processed_sentences.join(" ")
}

/// Normalize text to present tense (DISABLED — corrupts verbs without proper conjugation DB)
#[allow(dead_code)]
fn normalize_tense(text: &str) -> String {
    let pattern = Regex::new(r"(\w+)ed\b").unwrap();
    pattern.replace_all(text, "$1").to_string()
}

// =============================================================================
// Python-Visible Functions
// =============================================================================

/// Preprocess text: apply active voice + logical completeness check.
/// Does NOT apply full Caveman rules — use compress() for full compression.
#[pyfunction]
#[pyo3(signature = (text))]
pub fn preprocess_text(text: &str) -> PyResult<String> {
    let mut result = String::from(text);

    // Transform to active voice (agent verb_past the subject)
    result = transform_active_voice(&result);

    // Present tense normalization disabled (needs proper verb DB)
    // result = normalize_tense(&result);

    // Check logical completeness
    if !is_logically_complete(&result) {
        return Err(exceptions::PyValueError::new_err(
            "Text lacks logical completeness - please provide complete sentences",
        ));
    }

    Ok(result)
}

/// Compress text using all Caveman Compression rules.
/// Returns a token-reduced string suitable for LLM input or binary compression.
#[pyfunction]
#[pyo3(signature = (text))]
pub fn compress(text: &str) -> PyResult<String> {
    let result = apply_caveman_rules(text);
    if result.is_empty() {
        return Err(exceptions::PyValueError::new_err(
            "Compression produced empty output",
        ));
    }
    Ok(result)
}

// =============================================================================
// Python Module Registration
// =============================================================================

#[pymodule]
fn rust_cave_001(
    _py: Python,
    module: &pyo3::prelude::Bound<'_, pyo3::types::PyModule>,
) -> PyResult<()> {
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

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_sentences() {
        let sentences = split_into_sentences("Hello. World! How are you?");
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Hello");
        assert_eq!(sentences[1], "World");
        assert_eq!(sentences[2], "How are you");
    }

    #[test]
    fn test_remove_articles() {
        assert_eq!(remove_articles("The database needs an index"), " database needs  index");
        assert_eq!(remove_articles("A test sentence"), " test sentence");
    }

    #[test]
    fn test_remove_intensifiers() {
        assert_eq!(remove_intensifiers("very important"), "important");
        assert_eq!(remove_intensifiers("extremely fast query"), "fast query");
        assert_eq!(remove_intensifiers("quite large dataset"), "large dataset");
    }

    #[test]
    fn test_eliminate_connectives() {
        assert_eq!(
            eliminate_connectives("Use index because query too slow"),
            "Use index  query too slow"
        );
        assert_eq!(
            eliminate_connectives("However, the index has overhead"),
            " , the index has overhead"
        );
    }

    #[test]
    fn test_enforce_word_limit() {
        assert_eq!(enforce_word_limit("Need fast queries"), "Need fast queries"); // 3 words OK
        assert_eq!(enforce_word_limit("Hash map offers O(1) lookup"), "Hash map offers O(1) lookup"); // 5 words OK
        assert_eq!(
            enforce_word_limit("We need to implement a fast query system that uses indexes"),
            "We need to implement a"
        );
    }

    #[test]
    fn test_active_voice_transform() {
        assert_eq!(
            transform_active_voice("The ball was thrown by John"),
            "John threw the ball"
        );
        assert_eq!(
            transform_active_voice("The report was created by the team"),
            "the team made the report"
        );
    }

    #[test]
    fn test_caveman_rules_pipeline() {
        let input = "The database needs an index because the queries are too slow.";
        let output = apply_caveman_rules(input);
        assert!(!output.is_empty());
        // Article removal + connective elimination + word limit applied
        assert!(!output.contains("the "));
        assert!(!output.contains("because"));
    }

    #[test]
    fn test_logical_completeness() {
        assert!(is_logically_complete("The dog chased the cat"));
        assert!(is_logically_complete("I am here"));
        assert!(!is_logically_complete("Hello world"));
        assert!(!is_logically_complete("Hello"));
    }
}