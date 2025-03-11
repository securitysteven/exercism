/// Returns the number of steps required to reach 1 in the Collatz sequence for a given positive integer.
///
/// The Collatz conjecture (or 3n + 1 conjecture) posits that no matter the starting number,
/// the sequence will always eventually reach 1. If `n` is 0, the function returns `None` as
/// the sequence is undefined for 0.
///
/// # Arguments
///
/// * `n` - A positive integer (non-zero).
///
/// # Returns
///
/// * `Some(u64)` - The number of steps required to reach 1, or `None` if `n` is 0.
///
/// # Example
///
/// ```
/// let steps = collatz(6);
/// assert_eq!(steps, Some(8)); // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
/// ```
///
/// # Panics
///
/// Panics if `n` is 0.
pub fn collatz(mut n: u64) -> Option<u64> {
    // Early exit if `n` is 0, as the Collatz sequence is not defined for 0
    if n == 0 {
        return None;
    }

    // Direct return if `n` is already 1 (no steps needed)
    if n == 1 {
        return Some(0);
    }

    // Step counter
    let mut steps = 0;

    // Loop until we reach 1
    while n != 1 {
        if n % 2 == 0 {
            n /= 2; // If even, divide by 2
        } else {
            n = n * 3 + 1; // If odd, multiply by 3 and add 1
        }
        steps += 1; // Increment step count
    }

    Some(steps)
}