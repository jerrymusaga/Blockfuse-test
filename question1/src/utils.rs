
//
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // just checking for empty strings and initializing new one if none is found
    if strs.is_empty() {
        return String::new();
    }
    
    let first = &strs[0];

    /*
    Keep track of the current character position
    Compare characters across different strings
    Stop as soon as I find a mismatch
    */ 
    for (i, &ch) in first.as_bytes().iter().enumerate() {
        // Check if this character matches in all other strings at the same index
        for s in &strs[1..] {
            // If reached the end of any string or character doesn't match
            if i >= s.len() || s.as_bytes()[i] != ch {
                // Return the prefix up to this point
                return first[..i].to_string();
            }
        }
    }
    
    // If made it through the entire first string, it's the common prefix
    first.to_string()
}