/// Returns the number of steps required to reach 1, following the Collatz conjecture.
///
/// # Arguments
///
/// * `n` - A positive integer.
///
/// # Returns
///
/// * `Some(u64)` - The number of steps to reach 1, or `None` if `n` is 0.
///
/// # Example
///
/// ```
/// let steps = collatz(6);
/// assert_eq!(steps, Some(8)); // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
/// ```
pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None; // The Collatz sequence is not defined for 0.
    }

    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2; // Divide by 2 if the number is even
        } else {
            n = n * 3 + 1; // Multiply by 3 and add 1 if the number is odd
        }
        steps += 1; // Increment the step count
    }

    Some(steps) // Return the number of steps as Some(steps)
}
