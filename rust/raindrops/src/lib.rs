pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    // Append sounds based on divisibility
    [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .for_each(|&(divisor, sound)| {
            if n % divisor == 0 {
                result.push_str(sound);
            }
        });

    // Return number as string if no sounds were added
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}