use rust_cave_001::remove_articles;

fn main() {
    let texts = [
        "The database needs an index",
        "An apple a day",
        "A test",
    ];
    for text in texts {
        let result = remove_articles(text);
        println!("Input: {}", text);
        println!("Output: {}", result);
        println!("Contains 'a'? {}", result.to_lowercase().contains("a"));
        println!("Contains 'the'? {}", result.to_lowercase().contains("the"));
    }
}
