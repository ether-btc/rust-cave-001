use regex::Regex;

fn main() {
    let text = "The database needs an index";
    let pattern = Regex::new(r"(?i)\b(the|a|an)\b").unwrap();
    println!("Pattern: {:?}", pattern);
    let matches: Vec<&str> = pattern.find_iter(text).map(|m| m.as_str()).collect();
    println!("Matches: {:?}", matches);
    let result = pattern.replace_all(text, "");
    println!("Result: '{}'", result);
    println!("Contains 'the'? {}", result.to_lowercase().contains("the"));
}
