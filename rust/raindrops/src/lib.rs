pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    // Check divisibility and append sounds
    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }

    // If no sounds were added, return the number as a string
    if result.is_empty() {
        return n.to_string();
    }

    result
}