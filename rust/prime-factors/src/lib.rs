pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let numerator: u64 = n;
    let mut denominator = 2;

    while numerator != 1 {
        if numerator % denominator == 0 {
            factors.push(denominator);
        } else {
            denominator += 1;
        }
    }
    factors
}
