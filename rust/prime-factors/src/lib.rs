pub fn factors(n: u64) -> Vec<u64> {
    let factors: Vec<u64> = Vec::new();
    let numerator: n;
    let denominator = 2;

    while numerator != 1 {
        if numerator % denominator == 0 {
            factors.push(denominator);
        } else {
            denominator += 1;
        }
    }
}
