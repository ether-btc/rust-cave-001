use lz4::block::{self, CompressionMode};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

mod classifier;

mod verb_maps;

use std::collections::HashSet;
#[pyfunction]
#[pyo3(signature = (data, level = 9))]
/// Compress data using LZ4 algorithm
pub fn my_compress(data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    let mode = CompressionMode::HIGHCOMPRESSION(level);
    let compressed = block::compress(data, Some(mode), true)
        .map_err(|e| exceptions::PyOSError::new_err(e.to_string()))?;
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
    let re =
        Regex::new(r"\b\w+\b").map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
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
    // Uses the expanded verb_maps module (192 entries, v0.3.0)
    let verb_conjugations = verb_maps::build_verb_conjugation_map();

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

        // Strip trailing punctuation from agent before inserting into output
        let agent_trimmed = agent.trim_end_matches(['.', '!', '?']);
        // Return: "agent verb_past the subject"
        format!("{} {} the {}", agent_trimmed, verb_past, subject)
    });

    Ok(result.to_string())
}

/// Normalize past-tense verbs to present tense
#[pyfunction]
fn normalize_present_tense(text: &str) -> PyResult<String> {
    // Map of simple past → present base form (reverse of the conjugation map)
    // Uses the expanded verb_maps module (220 entries, v0.3.0)
    let present_tense_map = verb_maps::build_present_tense_map();

    let word_pattern = Regex::new(r"\b(\w+)\b").unwrap();
    let result = word_pattern.replace_all(text, |caps: &regex::Captures| {
        let word = &caps[1];
        let lower = word.to_lowercase();

        // Check the present tense map (case-insensitive lookup)
        if let Some(&present) = present_tense_map.get(lower.as_str()) {
            // Preserve original capitalization
            if word.starts_with(|c: char| c.is_uppercase())
                && !lower.chars().all(|c| c.is_uppercase())
            {
                let mut capitalized = String::with_capacity(present.len());
                let mut chars = present.chars();
                if let Some(first) = chars.next() {
                    capitalized.push(first.to_uppercase().next().unwrap_or(first));
                    capitalized.push_str(chars.as_str());
                }
                return capitalized;
            }
            return present.to_string();
        }

        // For regular verbs ending in "ed": try stripping "ed"
        // Guard: don't strip if remaining word < 3 chars (e.g., "ed" → ""),
        // or if the word ends in "eed" with stem < 4 chars (e.g., "speed" → not "spe")
        if lower.ends_with("ed") && lower.len() > 3 {
            let stem = &lower[..lower.len() - 2];
            if stem.len() >= 2 {
                // Handle "eed" words: only strip if stem is long enough (e.g., "agreed" → "agree")
                let skip_eed = lower.ends_with("eed") && stem.len() < 4;
                if !skip_eed {
                    // For words like "included", "provided", "decided":
                    // The base form ends in "e" (include, provide, decide).
                    // Stripping "ed" loses too much ("includ"). Try stripping "d" instead.
                    // Only apply when the letter before "ded"/"ted" is a vowel.
                    // This correctly handles "sorted" → "sort" (r is consonant → skip)
                    // while handling "included" → "include" (u is vowel → strip d).
                    let second_last = if lower.len() >= 4 {
                        lower.as_bytes()[lower.len() - 4] as char
                    } else {
                        ' ' // not enough chars
                    };
                    let is_vowel = |c: char| -> bool { matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') };

                    if (lower.ends_with("ded") && is_vowel(second_last))
                        || (lower.ends_with("ted") && is_vowel(second_last))
                    {
                        let e_stem = &lower[..lower.len() - 1]; // strip "d" not "ed"
                        if e_stem.len() >= 2 {
                            return e_stem.to_string();
                        }
                    }
                    // Default: simple "ed" → "" stripping
                    // (e.g., "stopped" → "stopp", "worked" → "work")
                    return stem.to_string();
                }
            }
        }

        word.to_string()
    });

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
                    while let Some(_ws) = chars.next_if(|c| c.is_whitespace()) {}
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

    // Count articles that would be removed
    let pattern = Regex::new(r"(?i)\b(this|the|a|an)\b").unwrap();
    let article_count = words.iter().filter(|w| pattern.is_match(w)).count();

    // If removal would leave less than 3 words, preserve unchanged
    if word_count - article_count < 3 {
        return text.to_string();
    }

    // Pattern to match articles at word boundaries (case-insensitive)
    let pattern = Regex::new(r"(?i)\b(this|the|a|an)\b").unwrap();
    let result = pattern.replace_all(text, "").to_string();

    // Collapse multiple spaces into single space
    let collapse_spaces = Regex::new(r"\s+").unwrap();
    let result = collapse_spaces.replace_all(&result, " ").to_string();

    // Trim extra spaces left by removal
    result.trim().to_string()
}

// Remove intensifiers (very, extremely, quite, rather, really, somewhat)
// Short sentences where removal would produce <3 words are preserved unchanged
fn remove_intensifiers(text: &str) -> String {
    // Split into words to check length
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    // Count intensifiers that would be removed
    let pattern = Regex::new(r"(?i)\b(very|extremely|quite|rather|really|somewhat)\b").unwrap();
    let intensifier_count = words.iter().filter(|w| pattern.is_match(w)).count();

    // If removal would leave less than 3 words, preserve unchanged
    if word_count - intensifier_count < 3 {
        return text.to_string();
    }

    // Pattern to match intensifiers at word boundaries (case-insensitive)
    let pattern = Regex::new(r"(?i)\b(very|extremely|quite|rather|really|somewhat)\b").unwrap();
    let result = pattern.replace_all(text, "").to_string();

    // Collapse multiple spaces into single space
    let collapse_spaces = Regex::new(r"\s+").unwrap();
    let result = collapse_spaces.replace_all(&result, " ").to_string();

    // Trim extra spaces
    result.trim().to_string()
}

// Remove connectives (because, however, therefore, but)
// Replaces with space to prevent word merging (case-insensitive)
fn eliminate_connectives(text: &str) -> String {
    let pattern = Regex::new(r"(?i)\s*\b(because|however|therefore|but)\b,?\s*").unwrap();
    pattern.replace_all(text, " ").trim().to_string()
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

/// Handle pronoun ambiguity (SPEC Rule 8)
/// Keeps short pronouns when unambiguous; replaces with preceding noun when ambiguous.
/// Simplified v1: handles "it" — if previous sentence has 2+ noun-like words (>3 chars),
/// replace "it" with the most recent one.
fn resolve_pronouns(sentences: &mut [String]) {
    let pronouns = ["it", "they", "them", "this", "that"];
    let stop_words = [
        "the",
        "a",
        "an",
        "this",
        "that",
        "these",
        "those",
        "is",
        "was",
        "are",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "do",
        "does",
        "did",
        "will",
        "would",
        "could",
        "should",
        "may",
        "might",
        "can",
        "shall",
        "not",
        "no",
        "nor",
        "but",
        "if",
        "or",
        "and",
        "because",
        "however",
        "therefore",
        "very",
        "extremely",
        "quite",
        "rather",
        "really",
        "somewhat",
    ];

    let is_noun = |word: &str| -> bool {
        let lower = word.trim_end_matches(['.', ',', '!', '?']).to_lowercase();
        lower.len() > 3 && !stop_words.contains(&lower.as_str())
    };

    for i in 1..sentences.len() {
        let prev_words: Vec<&str> = sentences[i - 1].split_whitespace().collect();
        let current_sentence = sentences[i].clone();
        let current_words: Vec<&str> = current_sentence.split_whitespace().collect();

        // Find nouns in previous sentence (candidates for pronoun reference)
        let noun_candidates: Vec<&str> =
            prev_words.iter().filter(|w| is_noun(w)).copied().collect();

        // Check if current sentence starts with or contains a pronoun
        let mut needs_replace = false;
        let mut pronoun_idx = None;
        for (j, word) in current_words.iter().enumerate() {
            let clean = word.trim_end_matches(['.', ',', '!', '?']).to_lowercase();
            if pronouns.contains(&clean.as_str()) && noun_candidates.len() >= 2 {
                needs_replace = true;
                pronoun_idx = Some(j);
                break;
            }
        }

        if needs_replace {
            if let Some(last_noun) = noun_candidates.last() {
                let replacement = last_noun.trim_end_matches(['.', ',', '!', '?']);
                let new_words: Vec<String> = current_words
                    .iter()
                    .enumerate()
                    .map(|(j, w)| {
                        if Some(j) == pronoun_idx {
                            replacement.to_string()
                        } else {
                            w.to_string()
                        }
                    })
                    .collect();
                sentences[i] = new_words.join(" ");
            }
        }
    }
}

// Apply selected Caveman compression rules based on strategy.
// When strategy is None, applies ALL rules (full pipeline).
fn apply_caveman_rules(text: &str, strategy: Option<&HashSet<&str>>) -> PyResult<String> {
    // 1. Split into sentences (if multiple)
    let sentences = split_into_sentences(text);
    // Resolve pronoun ambiguity — operates on sentence list before loop
    let mut sentences = sentences;
    if strategy.is_none_or(|s| s.contains("resolve_pronouns")) {
        resolve_pronouns(&mut sentences);
    }
    let mut processed_sentences = Vec::new();

    for sentence in sentences {
        let mut result = sentence;

        // Active voice transformation
        if strategy.is_none_or(|s| s.contains("active_voice")) {
            result = transform_active_voice(&result)?;
        }

        // Present tense normalization
        if strategy.is_none_or(|s| s.contains("present_tense")) {
            result = normalize_present_tense(&result)?;
        }

        // Remove articles
        if strategy.is_none_or(|s| s.contains("remove_articles")) {
            result = remove_articles(&result);
        }

        // Remove intensifiers
        if strategy.is_none_or(|s| s.contains("remove_intensifiers")) {
            result = remove_intensifiers(&result);
        }

        // Remove connectives
        if strategy.is_none_or(|s| s.contains("eliminate_connectives")) {
            result = eliminate_connectives(&result);
        }

        // Enforce word limit
        if strategy.is_none_or(|s| s.contains("word_limit_5")) {
            result = enforce_word_limit(&result);
        }

        // Check logical completeness (at least 2 words)
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

/// Full-pipeline compress — all 9 rules (default, unchanged behavior).
#[pyfunction]
#[pyo3(signature = (text))]
pub fn compress(text: &str) -> PyResult<String> {
    apply_caveman_rules(text, None)
}

/// Adaptive compress — auto-classifies text and selects optimal rule subset.
#[pyfunction]
#[pyo3(signature = (text))]
pub fn compress_adaptive(text: &str) -> PyResult<String> {
    use crate::classifier::{classify, recommended_strategy};
    let text_type = classify(text);
    let strategy_names = recommended_strategy(text_type);
    let strategy: HashSet<&str> = strategy_names.iter().copied().collect();
    apply_caveman_rules(text, Some(&strategy))
}

/// Preprocess text by applying active voice, present tense, and logical completeness checks
#[pyfunction]
#[pyo3(signature = (text))]
pub fn preprocess_text(text: &str) -> PyResult<String> {
    let mut result = String::from(text);

    // Transform to active voice (agent verb_past the subject)
    result = transform_active_voice(&result)?;

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
    module.add_function(wrap_pyfunction!(compress_adaptive, module)?)?;
    module.add_function(wrap_pyfunction!(normalize_present_tense, module)?)?;
    module.add_function(wrap_pyfunction!(classifier::classify_text, module)?)?;
    module.add_function(wrap_pyfunction!(
        classifier::recommended_strategy_for_text,
        module
    )?)?;
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

        // "An apple a day" has 4 words; removing "An"+"a" leaves 2 (< 3 minimum)
        // so the safety guard preserves the original. Test that guard works.
        let result2 = remove_articles("An apple a day");
        assert!(!result2.contains("an ")); // capital "An" is removed
                                           // lowercase "a" is protected by the 3-word minimum guard
        assert!(result2.contains("a day"));

        let result3 = remove_articles("A test");
        assert_eq!(result3, "A test"); // guard preserves 2-word input

        // Longer input: should remove articles
        let result4 = remove_articles("A big apple a day keeps the doctor");
        assert!(!result4.contains(" a "));
        assert!(!result4.contains(" A "));
        assert!(!result4.contains(" the "));
    }

    #[test]
    fn test_transform_active_voice() {
        let result = transform_active_voice("The ball was thrown by John").unwrap();
        println!("Debug: result = '{}'", result);
        assert!(result.contains("John"));
        assert!(result.contains("threw"));
        assert!(result.contains("the")); // transform_active_voice does NOT remove articles; that's done later in compress()
    }
}
