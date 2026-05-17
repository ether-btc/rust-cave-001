use lz4::block::{self, CompressionMode};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

mod classifier;

mod verb_maps;

mod error;

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
    use std::sync::OnceLock;

    // Map of simple past → present base form (reverse of the conjugation map)
    // Uses the expanded verb_maps module (220 entries, v0.3.0)
    let present_tense_map = verb_maps::build_present_tense_map();

    static WORD_PATTERN: OnceLock<Regex> = OnceLock::new();
    let word_pattern = WORD_PATTERN.get_or_init(|| Regex::new(r"\b(\w+)\b").unwrap());

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
    use std::sync::OnceLock;
    // Simplified check: at least two words
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| Regex::new(r"\b\w+\b\s+\b\w+\b").unwrap());
    pattern.is_match(text)
}

// Split text into sentences based on punctuation (. ! ?)
// With basic abbreviation protection to avoid splitting on "Dr.", "U.S.A.", etc.
// Uses OnceLock to compile the abbreviation regex once.
fn split_into_sentences(text: &str) -> Vec<String> {
    use std::sync::OnceLock;

    static ABBREVIATION_PATTERN: OnceLock<Regex> = OnceLock::new();
    let abbr_re = ABBREVIATION_PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)\b(dr|mr|mrs|ms|prof|sr|jr|st|ave|blvd|etc|vs|inc|ltd|co|dept|est|govt|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec|u\.s\.?a?|e\.g|i\.e|al)\.$"
        ).unwrap()
    });

    let mut sentences = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        current.push(c);

        // Check for sentence-ending punctuation followed by space or end of string
        if c == '.' || c == '!' || c == '?' {
            // Abbreviation check: if the word before '.' is a known abbreviation, don't split
            if c == '.' {
                let trimmed = current.trim();
                if abbr_re.is_match(trimmed) {
                    continue; // Not a sentence boundary
                }
            }

            // Look ahead: if next char is whitespace/end, this is a sentence boundary
            match chars.peek() {
                Some(&next) if next.is_whitespace() => {
                    let trimmed = current.trim();
                    // If sentence ends with "..." followed by more text, treat as
                    // mid-ellipsis and don't split (e.g. "Hello... This" → one sentence)
                    if trimmed.ends_with("...") {
                        continue; // keep accumulating — ellipsis is not a terminal boundary
                    }
                    sentences.push(trimmed.to_string());
                    current.clear();
                    while let Some(_ws) = chars.next_if(|c| c.is_whitespace()) {}
                }
                None => {
                    sentences.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    // Not a boundary (e.g., part of "..." or decimal number)
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
    use std::sync::OnceLock;

    static ARTICLE_PATTERN: OnceLock<Regex> = OnceLock::new();
    static SPACE_PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = ARTICLE_PATTERN.get_or_init(|| Regex::new(r"(?i)\b(this|the|a|an)\b").unwrap());
    let collapse_spaces = SPACE_PATTERN.get_or_init(|| Regex::new(r"\s+").unwrap());

    // Split into words to check length
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    // Count articles that would be removed
    let article_count = words.iter().filter(|w| pattern.is_match(w)).count();

    // If removal would leave less than 3 words, preserve unchanged
    if word_count - article_count < 3 {
        return text.to_string();
    }

    // Apply article removal
    let result = pattern.replace_all(text, "").to_string();

    // Collapse multiple spaces into single space
    let result = collapse_spaces.replace_all(&result, " ").to_string();

    // Trim extra spaces left by removal
    result.trim().to_string()
}

// Expand English contractions into full forms (e.g., "don't" → "do not").
// Must run early so downstream rules can process the expanded forms.
// Uses OnceLock static cache to compile regexes once (not 63× per call).
fn expand_contractions(text: &str) -> String {
    use std::sync::OnceLock;

    static CONTRACTION_REGEXES: OnceLock<Vec<(regex::Regex, &'static str)>> = OnceLock::new();
    let regexes = CONTRACTION_REGEXES.get_or_init(|| {
        const PAIRS: &[(&str, &str)] = &[
            // ── n't contractions ──
            ("don't", "do not"),
            ("doesn't", "does not"),
            ("didn't", "did not"),
            ("won't", "will not"),
            ("wouldn't", "would not"),
            ("can't", "cannot"),
            ("couldn't", "could not"),
            ("shouldn't", "should not"),
            ("mightn't", "might not"),
            ("mustn't", "must not"),
            ("isn't", "is not"),
            ("aren't", "are not"),
            ("wasn't", "was not"),
            ("weren't", "were not"),
            ("hasn't", "has not"),
            ("haven't", "have not"),
            ("hadn't", "had not"),
            ("needn't", "need not"),
            ("daren't", "dare not"),
            // ── 's contractions ──
            ("it's", "it is"),
            ("he's", "he is"),
            ("she's", "she is"),
            ("that's", "that is"),
            ("what's", "what is"),
            ("there's", "there is"),
            ("who's", "who is"),
            ("here's", "here is"),
            ("where's", "where is"),
            ("how's", "how is"),
            ("let's", "let us"),
            // ── 'm / 're / 've ──
            ("i'm", "i am"),
            ("we're", "we are"),
            ("they're", "they are"),
            ("you're", "you are"),
            ("i've", "i have"),
            ("we've", "we have"),
            ("they've", "they have"),
            ("you've", "you have"),
            // ── 'll ──
            ("i'll", "i will"),
            ("we'll", "we will"),
            ("they'll", "they will"),
            ("you'll", "you will"),
            ("he'll", "he will"),
            ("she'll", "she will"),
            ("it'll", "it will"),
            ("that'll", "that will"),
            // ── 'd ──
            ("i'd", "i would"),
            ("we'd", "we would"),
            ("they'd", "they would"),
            ("you'd", "you would"),
            ("he'd", "he would"),
            ("she'd", "she would"),
            ("it'd", "it would"),
            // ── Informal ──
            ("gonna", "going to"),
            ("wanna", "want to"),
            ("gotta", "got to"),
            ("kinda", "kind of"),
            ("sorta", "sort of"),
            ("outta", "out of"),
            ("oughta", "ought to"),
            ("lemme", "let me"),
            ("gimme", "give me"),
            ("dunno", "do not know"),
            ("cmon", "come on"),
            ("cos", "because"),
        ];
        PAIRS
            .iter()
            .map(|(c, e)| {
                let pattern = format!(r"(?i)\b{}\b", regex::escape(c));
                (regex::Regex::new(&pattern).unwrap(), *e)
            })
            .collect()
    });

    let mut result = text.to_string();
    for (re, expansion) in regexes {
        result = re.replace_all(&result, *expansion).to_string();
    }
    result
}

// Remove copular "be" verbs (is, are, was, were, am, be, been, being) from text
fn remove_copular_be(text: &str) -> String {
    use std::sync::OnceLock;

    static BE_PATTERN: OnceLock<Regex> = OnceLock::new();
    static SPACE_PATTERN: OnceLock<Regex> = OnceLock::new();
    let be_verb_pattern = BE_PATTERN
        .get_or_init(|| Regex::new(r"(?i)\b(is|are|was|were|am|be|been|being)\b").unwrap());
    let re_spaces = SPACE_PATTERN.get_or_init(|| Regex::new(r"\s+").unwrap());

    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    // Count "be" verbs that would be removed
    let be_count = words.iter().filter(|w| be_verb_pattern.is_match(w)).count();

    // If removal would leave less than 2 words, preserve unchanged
    if word_count - be_count < 2 {
        return text.to_string();
    }

    let result = be_verb_pattern
        .replace_all(text, |caps: &regex::Captures| {
            let word = caps.get(0).unwrap().as_str();
            // Acronym protection: skip removal if word is fully uppercase (e.g., IS, BE, AM)
            if word.chars().all(|c| c.is_uppercase()) {
                return word.to_string();
            }
            String::new()
        })
        .to_string();
    // Collapse multiple spaces
    re_spaces.replace_all(&result, " ").trim().to_string()
}

// Remove intensifiers (very, extremely, quite, rather, really, somewhat)
// Short sentences where removal would produce <3 words are preserved unchanged
fn remove_intensifiers(text: &str) -> String {
    use std::sync::OnceLock;

    static INTENSIFIER_PATTERN: OnceLock<Regex> = OnceLock::new();
    static SPACE_PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = INTENSIFIER_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)\b(very|extremely|quite|rather|really|somewhat)\b").unwrap()
    });
    let collapse_spaces = SPACE_PATTERN.get_or_init(|| Regex::new(r"\s+").unwrap());

    // Split into words to check length
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    // Count intensifiers that would be removed
    let intensifier_count = words.iter().filter(|w| pattern.is_match(w)).count();

    // If removal would leave less than 3 words, preserve unchanged
    if word_count - intensifier_count < 3 {
        return text.to_string();
    }

    // Apply intensifier removal
    let result = pattern.replace_all(text, "").to_string();

    // Collapse multiple spaces into single space
    let result = collapse_spaces.replace_all(&result, " ").to_string();

    // Trim extra spaces
    result.trim().to_string()
}

// Remove connectives (coordinating/subordinating conjunctions and transition words)
// Replaces with space to prevent word merging (case-insensitive)
// Covers: because, however, therefore, but, and, or, although, since, unless, while, whereas
fn eliminate_connectives(text: &str) -> String {
    use std::sync::OnceLock;

    static CONNECTIVE_PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = CONNECTIVE_PATTERN.get_or_init(|| {
        Regex::new(r"(?i)\s*\b(because|however|therefore|but|and|or|although|since|unless|while|whereas)\b,?\s*").unwrap()
    });

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

        // Contraction expansion — runs first so pipeline processes full forms
        if strategy.is_none_or(|s| s.contains("expand_contractions")) {
            result = expand_contractions(&result);
        }

        // Active voice transformation
        if strategy.is_none_or(|s| s.contains("active_voice")) {
            result = transform_active_voice(&result)?;
        }

        // Present tense normalization
        if strategy.is_none_or(|s| s.contains("present_tense")) {
            result = normalize_present_tense(&result)?;
        }

        // Remove articles — runs first (highest min-word guard: 3)
        if strategy.is_none_or(|s| s.contains("remove_articles")) {
            result = remove_articles(&result);
        }

        // Remove intensifiers — runs before be_removal (min-word guard: 3)
        if strategy.is_none_or(|s| s.contains("remove_intensifiers")) {
            result = remove_intensifiers(&result);
        }

        // Remove copular "be" verbs (is, are, was, were, am, be, been, being)
        // Runs after articles+intensifiers so higher-guard rules fire when word count is highest
        if strategy.is_none_or(|s| s.contains("remove_copular_be")) {
            result = remove_copular_be(&result);
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
            return Err(crate::error::CompressionError::TooShort(result).into_pyerr());
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
        return Err(crate::error::CompressionError::TooShort(result).into_pyerr());
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
        assert!(!is_logically_complete("a"));
        assert!(is_logically_complete("Testing logical completeness here"));
    }

    #[test]
    fn test_remove_articles() {
        let result1 = remove_articles("The database needs an index");
        assert!(!result1.to_lowercase().contains("the"));

        let result2 = remove_articles("An apple a day");
        assert!(!result2.contains("an "));
        assert!(result2.contains("a day"));

        let result3 = remove_articles("A test");
        assert_eq!(result3, "A test");

        let result4 = remove_articles("A big apple a day keeps the doctor");
        assert!(!result4.contains(" a "));
        assert!(!result4.contains(" A "));
        assert!(!result4.contains(" the "));
    }

    #[test]
    fn test_transform_active_voice() {
        let result = transform_active_voice("The ball was thrown by John").unwrap();
        assert!(result.contains("John"));
        assert!(result.contains("threw"));
        assert!(result.contains("the"));
    }

    #[test]
    fn test_expand_contractions() {
        assert_eq!(expand_contractions("don't"), "do not");
        assert_eq!(expand_contractions("can't"), "cannot");
        assert_eq!(expand_contractions("won't"), "will not");
        assert_eq!(expand_contractions("it's"), "it is");
        assert_eq!(expand_contractions("i'm"), "i am");
        assert_eq!(expand_contractions("they're"), "they are");
        assert_eq!(expand_contractions("i've"), "i have");
        assert_eq!(expand_contractions("he'll"), "he will");
        assert_eq!(expand_contractions("she'd"), "she would");
        assert_eq!(expand_contractions("we'd have"), "we would have");
        // No-op for regular text
        assert_eq!(expand_contractions("hello world"), "hello world");
        // Case handling — lowercase match (function is case-sensitive)
        assert_eq!(expand_contractions("Don't"), "do not");
        assert_eq!(expand_contractions("I'm here"), "i am here");
    }

    #[test]
    fn test_expand_contractions_edge_cases() {
        // Unicode — accented chars: function uses ASCII patterns, they pass through
        assert_eq!(expand_contractions("café's good"), "café's good"); // no ASCII match
                                                                       // Possessive 's — correctly NOT matched (only specific forms listed)
        assert_eq!(expand_contractions("cat's tail"), "cat's tail");
        // Multi-contraction sentence
        assert_eq!(
            expand_contractions("I don't think it's working"),
            "I do not think it is working"
        );
        // Empty/near-empty
        assert_eq!(expand_contractions(""), "");
        assert_eq!(expand_contractions("x"), "x");
        // Numbers with apostrophes
        assert_eq!(expand_contractions("'80s"), "'80s"); // no match
                                                         // Already expanded
        assert_eq!(expand_contractions("do not"), "do not");
    }

    #[test]
    fn test_remove_intensifiers() {
        let result = remove_intensifiers("The extremely fast query");
        assert!(!result.contains("extremely"));

        // Short sentence protection
        assert_eq!(remove_intensifiers("very fast"), "very fast");

        // Normal removal in long sentence
        let result2 = remove_intensifiers("This is a really fast system indeed");
        assert!(!result2.contains("really"));
    }

    #[test]
    fn test_eliminate_connectives() {
        assert!(!eliminate_connectives("Use index because query slow").contains("because"));
        assert!(!eliminate_connectives("However the system is slow.").contains("however"));
        assert!(!eliminate_connectives("Query slow therefore use index").contains("therefore"));
        assert!(!eliminate_connectives("Index helps but uses space").contains("but"));
        // No word merging
        assert!(!eliminate_connectives("raining therefore we").contains("rainingtherefore"));
    }

    #[test]
    fn test_enforce_word_limit() {
        assert_eq!(enforce_word_limit("short text"), "short text");
        let truncated = enforce_word_limit("This is a very long sentence that should be truncated");
        assert!(truncated.split_whitespace().count() <= 5);
        // Comma split
        let result = enforce_word_limit("Take first clause, discard the rest of this sentence");
        assert!(result.len() < 30);
    }

    #[test]
    fn test_split_into_sentences() {
        let result = split_into_sentences("Hello world. This is a test.");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "Hello world.");
        assert_eq!(result[1], "This is a test.");

        let single = split_into_sentences("Just one sentence");
        assert_eq!(single.len(), 1);

        let empty = split_into_sentences("");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_split_abbreviation_protection() {
        // Abbreviation "Dr." should NOT cause a split
        let result = split_into_sentences("Dr. Smith went to the store.");
        assert_eq!(
            result.len(),
            1,
            "Abbreviation 'Dr.' should not split: {:?}",
            result
        );

        // "U.S.A." should not cause splits
        let result2 = split_into_sentences("The U.S.A. is a country.");
        assert_eq!(result2.len(), 1, "'U.S.A.' should not split: {:?}", result2);

        // "e.g." and "i.e." should not cause splits
        let result3 = split_into_sentences("Use tools e.g. hammers.");
        assert_eq!(result3.len(), 1, "'e.g.' should not split: {:?}", result3);

        // Normal sentences still split correctly
        let result4 = split_into_sentences("First sentence. Second sentence.");
        assert_eq!(
            result4.len(),
            2,
            "Normal sentences should split: {:?}",
            result4
        );
    }
}
