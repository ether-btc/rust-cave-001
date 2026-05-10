use regex::Regex;

fn is_logically_complete(text: &str) -> bool {
    let pattern = Regex::new(r"\b\w+\b\s+\b\w+\b").unwrap();
    pattern.is_match(text)
}

fn main() {
    let text = "Hello world";
    let result = is_logically_complete(text);
    println!("Testing: {}", text);
    println!("Result: {}", result);
}
