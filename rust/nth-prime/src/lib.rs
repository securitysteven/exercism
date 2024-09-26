pub fn nth(n: usize) -> Option<u32> {
    // Call the list_primes function with a sufficiently high number
    let primes = list_primes(n); // 100 is enough to include the first 25 primes

}
fn list_primes(max: u32) -> Vec<u32> {
    if max < 2 {
        return vec![]; // No primes less than 2
    }

    let mut primes = vec![2];

    // Iterate only through odd numbers starting from 3
    for n in (3..=max).step_by(2) {
        let is_prime = primes.iter().take_while(|&&p| p * p <= n).all(|&p| n % p != 0);
        if is_prime {
            primes.push(n);
        }
    }

    primes
}
