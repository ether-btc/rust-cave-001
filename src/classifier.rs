/// Input text classifier — detects text type for adaptive rule selection.
///
/// Uses simple heuristics (no ML dependency) to classify text into types
/// that respond optimally to different compression rule subsets.
///
/// This is the first component of the self-learning framework.
/// The strategy selector (which maps type → rule subset) comes next,
/// informed by benchmark oracle data.
use regex::Regex;

/// Categories of text input that the compression pipeline may encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextType {
    /// Technical documentation, code references, API specs, factual reports
    Technical,
    /// Natural conversation, chat messages, informal discussion
    Conversational,
    /// Academic papers, dense analytical content, long-form analysis
    Academic,
    /// Dialogue, very short messages, Q&A pairs
    Dialogue,
    /// Already minimal — short, few articles/connectives, likely pre-compressed
    AlreadyMinimal,
    /// Fallback — apply full pipeline
    Mixed,
}

impl TextType {
    pub fn label(&self) -> &'static str {
        match self {
            TextType::Technical => "technical",
            TextType::Conversational => "conversational",
            TextType::Academic => "academic",
            TextType::Dialogue => "dialogue",
            TextType::AlreadyMinimal => "minimal",
            TextType::Mixed => "mixed",
        }
    }
}

/// Classify input text into a TextType based on content heuristics.
///
/// Scoring dimensions:
/// - avg_word_len: average word length (academic = long, dialogue = short)
/// - connective_density: proportion of connectives (because, however, etc.)
/// - article_density: proportion of articles (the, a, an)
/// - pronoun_density: proportion of pronouns
/// - code_like_score: proportion of code-like patterns (camelCase, =, (), {})
/// - sentence_count: number of sentences
/// - short_sentence_ratio: proportion of sentences under 5 words
pub fn classify(text: &str) -> TextType {
    let text = text.trim();
    if text.is_empty() {
        return TextType::Mixed;
    }

    let word_count = count_words(text);
    if word_count == 0 {
        return TextType::Mixed;
    }

    // Heuristic features
    let avg_word_len = average_word_length(text);
    let sentence_count = count_sentences(text);

    // Density scores (normalized per 100 words)
    let scale = 100.0 / word_count as f64;
    let connective_density = (count_pattern(
        text,
        r"(?i)\b(because|however|therefore|but|although|since|thus|hence|moreover|nevertheless)\b",
    ) as f64
        * scale) as usize;
    let article_density =
        (count_pattern(text, r"(?i)\b(the|a|an|this|these|those)\b") as f64 * scale) as usize;
    let pronoun_density = (count_pattern(
        text,
        r"(?i)\b(i|you|he|she|it|we|they|me|him|her|us|them|my|your|his|its|our|their)\b",
    ) as f64
        * scale) as usize;
    let number_density = (count_pattern(text, r"\b\d+\b") as f64 * scale) as usize;
    let code_like_score = (count_pattern(text, r"[{}=><&|+\-*/%]") as f64 * scale) as usize;
    let colon_semicolon_density = (text.matches(':').count() as f64 * scale) as usize;
    let paren_density = (text.matches('(').count() as f64 * scale) as usize;
    let short_sentence_ratio = if sentence_count > 0.0 {
        let sentences: Vec<&str> = text
            .split(['.', '!', '?'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        let total = sentences.len() as f64;
        if total > 0.0 {
            let short = sentences
                .iter()
                .filter(|s| s.split_whitespace().count() <= 5)
                .count() as f64;
            short / total
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Already-minimal check: no articles, no connectives, very short
    let has_articles = article_density > 0;
    let has_connectives = connective_density > 0;
    let has_pronouns = pronoun_density > 0;
    let very_short = word_count <= 4;

    if !has_articles && !has_connectives && !has_pronouns && very_short {
        return TextType::AlreadyMinimal;
    }

    // Academic: long words, semicolons/colons, connectives — strongest structural signal
    if avg_word_len > 5.5 && colon_semicolon_density > 1 && connective_density > 2 {
        return TextType::Academic;
    }

    // Technical: code-like patterns, parentheses, numbers, or technical keywords
    let tech_keywords = r"(?i)\b(api|query|database|index|hash|map|function|class|method|string|int|bool|array|list|server|client|request|response|endpoint|route|config|schema|table|column|row|parser|compiler|runtime|binary|buffer|cache|thread|socket|protocol|null|undefined|async|await|import|export|module|package|library|syntax|regex|lambda|callback|promise|loop|iteration|recursion|algorithm|def|impl|struct|enum|trait|fn|let|mut|pub|static|const|return)\b";
    let tech_keyword_score = count_pattern(text, tech_keywords) as f64 * scale;

    if code_like_score > 2
        || paren_density > 1
        || (number_density > 1 && avg_word_len > 4.5)
        || tech_keyword_score > 3.0
    {
        return TextType::Technical;
    }

    // Dialogue check: very short sentences, high pronoun density, simple vocab,
    // must be a genuinely brief exchange (total text < 15 words)
    if short_sentence_ratio > 0.6 && pronoun_density > 3 && avg_word_len < 4.5 && word_count < 15 {
        return TextType::Dialogue;
    }

    // Conversational: high pronouns and avg word len < 5, OR conversational greetings
    let has_greeting = count_pattern(
        text,
        r"(?i)\b(hi|hey|hello|thanks|thank|please|sorry|yeah|okay|ok|yep|nope|oh|ah|wow|hmm|um|uh)\b",
    ) as f64
        * scale;
    if (pronoun_density > 4 && avg_word_len < 5.0) || has_greeting > 1.0 {
        return TextType::Conversational;
    }

    // Fallback: academic if avg word is long
    if avg_word_len > 5.2 {
        return TextType::Academic;
    }

    TextType::Mixed
}

/// Return recommended rule subset for a given text type.
/// Returns None to indicate "use default full pipeline".
pub fn recommended_strategy(text_type: TextType) -> &'static [&'static str] {
    match text_type {
        // Technical: remove articles + connectives aggressively, keep numbers,
        // word limit to 5, active voice, present tense
        TextType::Technical => &[
            "split_sentences",
            "resolve_pronouns",
            "active_voice",
            "present_tense",
            "remove_articles",
            "remove_intensifiers",
            "eliminate_connectives",
            "word_limit_5",
            "logical_completeness",
        ],
        // Conversational: remove intensifiers + connectives, keep pronouns
        TextType::Conversational => &[
            "split_sentences",
            "present_tense",
            "remove_intensifiers",
            "eliminate_connectives",
            "word_limit_5",
            "logical_completeness",
        ],
        // Academic: heavy connective elimination, present tense, word limit
        TextType::Academic => &[
            "split_sentences",
            "active_voice",
            "present_tense",
            "remove_articles",
            "remove_intensifiers",
            "eliminate_connectives",
            "word_limit_5",
            "logical_completeness",
        ],
        // Dialogue: minimal processing, keep natural flow
        TextType::Dialogue => &[
            "split_sentences",
            "remove_intensifiers",
            "eliminate_connectives",
            "word_limit_5",
        ],
        // Already minimal: no processing needed
        TextType::AlreadyMinimal => &["logical_completeness"],
        // Mixed / unknown: full pipeline
        TextType::Mixed => &[
            "split_sentences",
            "resolve_pronouns",
            "active_voice",
            "present_tense",
            "remove_articles",
            "remove_intensifiers",
            "eliminate_connectives",
            "word_limit_5",
            "logical_completeness",
        ],
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return 0.0;
    }
    let total: usize = words.iter().map(|w| w.len()).sum();
    total as f64 / words.len() as f64
}

fn count_sentences(text: &str) -> f64 {
    let re = Regex::new(r"[.!?](\s|$)").unwrap();
    re.find_iter(text).count().max(1) as f64
}

fn count_pattern(text: &str, pattern: &str) -> usize {
    Regex::new(pattern).map_or(0, |re| re.find_iter(text).count())
}

use pyo3::prelude::*;

/// Classify input text and return the text type label as a string.
#[pyfunction]
pub fn classify_text(text: &str) -> String {
    classify(text).label().to_string()
}

/// Return the recommended compression strategy for a text type.
#[pyfunction]
pub fn recommended_strategy_for_text(text: &str) -> Vec<String> {
    let text_type = classify(text);
    recommended_strategy(text_type)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_technical_code() {
        let text = "Hash map offers O(1) lookup. The function parse_input() returns a Result type. Use index 5 for fast access.";
        assert_eq!(classify(text), TextType::Technical);
    }

    #[test]
    fn test_classify_conversational() {
        let text = "Hey, I really think we should look into this. It seems quite fast. You know what I mean?";
        assert_eq!(classify(text), TextType::Conversational);
    }

    #[test]
    fn test_classify_academic() {
        let text = "The aforementioned methodology demonstrates significant improvement; however, further research is required to validate these findings across diverse populations. Consequently, we recommend a longitudinal approach.";
        assert_eq!(classify(text), TextType::Academic);
    }

    #[test]
    fn test_classify_dialogue() {
        let text = "Hi! How are you? I am fine. Good!";
        assert_eq!(classify(text), TextType::Dialogue);
    }

    #[test]
    fn test_classify_already_minimal() {
        let text = "Need fast queries";
        assert_eq!(classify(text), TextType::AlreadyMinimal);
    }

    #[test]
    fn test_classify_empty() {
        assert_eq!(classify(""), TextType::Mixed);
    }

    #[test]
    fn test_classify_benchmark_samples() {
        // Match the 9 benchmark text types
        struct Sample {
            text: &'static str,
            expected: TextType,
        }

        let samples = vec![
            // Technical short: "The database needs an index because the queries are too slow."
            Sample {
                text: "The database needs an index because the queries are too slow.",
                expected: TextType::Technical,
            },
            // Conversational: longer interaction
            Sample {
                text: "Hey! How is everything going with the new system?",
                expected: TextType::Conversational,
            },
            // Already minimal
            Sample {
                text: "Need index",
                expected: TextType::AlreadyMinimal,
            },
        ];

        for s in samples {
            let result = classify(s.text);
            assert_eq!(result, s.expected, "Failed for: {}", s.text);
        }
    }

    #[test]
    fn test_recommended_strategy_all_types() {
        for t in &[
            TextType::Technical,
            TextType::Conversational,
            TextType::Academic,
            TextType::Dialogue,
            TextType::AlreadyMinimal,
            TextType::Mixed,
        ] {
            let strategy = recommended_strategy(*t);
            assert!(
                !strategy.is_empty(),
                "Strategy for {:?} should not be empty",
                t
            );
        }
    }

    #[test]
    fn test_strategy_variation() {
        // Verify different types get different strategies
        let tech = recommended_strategy(TextType::Technical);
        let minimal = recommended_strategy(TextType::AlreadyMinimal);
        assert_ne!(tech.len(), minimal.len());
    }
}
