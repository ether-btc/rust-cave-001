use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        return;
    }
    let text = &args[1];

    // Test the pattern directly
    let pattern = Regex::new(r#"\b\w+\b.*\b\w+\b.*\b\w+\b"#).unwrap();
    let matches = pattern.is_match(text);
    println!("Pattern match for '{}': {}", text, matches);

    // Also test with some variations
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Word count: {}", words.len());
    println!("Words: {:?}", words);
}
