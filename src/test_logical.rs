use rust_cave_001::is_logically_complete;

fn main() {
    let text = "Hello world";
    let result = is_logically_complete(text);
    println!("Testing: {}", text);
    println!("Result: {}", result);
}
