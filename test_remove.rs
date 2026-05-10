use regex::Regex;

fn remove_articles(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    
    if word_count < 3 {
        return text.to_string();
    }
    
    let pattern = Regex::new(r"\b(the|a|an)\b").unwrap();
    let result = pattern.replace_all(text, "").to_string();
    
    result.trim().to_string()
}

fn main() {
    let text = "The database needs an index";
    let result = remove_articles(text);
    println!("Input: {}", text);
    println!("Output: {}", result);
    println!("Contains 'the'? {}", result.to_lowercase().contains("the"));
}
