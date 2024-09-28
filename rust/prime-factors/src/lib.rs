pub fn factors(n: u64) -> Vec<u64> {
    let factors: Vec<u64> = Vec::new();
    let numerator: n;
    let denominator = 2;

    while numerator != 1 {
        if is_whole_number(numerator, denominator) {
            factors.push(denominator);
        } else {
            denominator += 1;
        }
    }
}

fn is_whole_number(numerator: i32, denominator: i32) -> bool {
    numerator % denominator == 0
}