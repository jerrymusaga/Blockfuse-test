mod utils;
// Example usage
fn main() {
    let example1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Example 1 result: {}", utils::longest_common_prefix(example1));
    
    let example2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    println!("Example 2 result: {}", utils::longest_common_prefix(example2));
}


