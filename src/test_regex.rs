use regex::Regex;

fn main() {
    let text = "An apple a day";
    let pattern = Regex::new(r"(?i)\b(the|a|an)\b").unwrap();
    let matches: Vec<&str> = pattern.find_iter(text).map(|m| m.as_str()).collect();
    println!("Text: {}", text);
    println!("Matches: {:?}", matches);
    let result = pattern.replace_all(text, "");
    println!("Result: '{}'", result);
}
