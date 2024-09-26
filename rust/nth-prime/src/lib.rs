pub fn nth(n: u32) -> Option<u32> {
    let mut primes: Vec<u32> = vec![];
    let mut candidate = 2;

    while primes.len() <= n as usize {
        // Check if candidate is prime
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes.last().copied() // Return the last prime found as an Option
}
