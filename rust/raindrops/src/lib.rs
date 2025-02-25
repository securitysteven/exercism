
pub fn raindrops(n: u32) -> String {
    //is divisible by 3, add "Pling" to the result.
    if n % 3 == 0 || n % 5 == 0 {
        format!("Pling{}", n)
    }
    // is divisible by 5, add "Plang" to the result.
    else if n % 7 == 0 {
        format!("Plang{}", n)
    }
    // is divisible by 7, add "Plong" to the result.
    else if n % 3 == 0 {
        format!("Plong{}", n)
    }
    // is not divisible by 3, 5, or 7, the result should be the number as a string.
    else {
        n.to_string()
    }
}
