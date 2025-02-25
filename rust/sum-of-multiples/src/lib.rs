/// Calculates the sum of all unique multiples of the given factors below the specified limit.
///
/// # Parameters
/// - `limit`: The upper bound (exclusive) for which multiples are considered.
/// - `factors`: A slice of factors to consider. Multiples of each factor will be summed. Zeroes will be ignored.
///
/// # Returns
/// - The sum of all unique multiples of the factors less than the limit.
///
/// # Examples
///
/// ```
/// let result = sum_of_multiples(10, &[3, 5]);
/// assert_eq!(result, 23);  // 3, 5, 6, 9 are the multiples below 10, and their sum is 23
/// ```
///
/// ```
/// let result = sum_of_multiples(1, &[0, 5]);
/// assert_eq!(result, 0);  // 0 is ignored, and no multiples of 5 below 1
/// ```
///
/// # Panics
/// This function does not panic under normal circumstances. If an invalid state were encountered,
/// such as a factor being zero (which is ignored), it would be silently skipped.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let valid_factors = factors.iter().filter(|&&factor| factor != 0);

    let mut multiples = std::collections::HashSet::new();

    for &factor in valid_factors {
        for multiple in (factor..limit).step_by(factor as usize) {
            multiples.insert(multiple);
        }
    }

    multiples.iter().copied().sum()
}