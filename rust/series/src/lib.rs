/// Generates a series of substrings of a given length from the input string.
///
/// # Arguments
/// * `digits`: A string slice to extract substrings from.
/// * `len`: The length of each substring in the resulting series.
///
/// # Returns
/// A vector of substrings of the specified length. If `len` is zero or greater than
/// the length of `digits`, an empty vector is returned.
///
/// # Examples
/// ```
/// let result = series("12345", 3);
/// assert_eq!(result, ["123", "234", "345"]);
///
/// let result = series("12345", 0);
/// assert_eq!(result, []);
///
/// let result = series("12345", 6);
/// assert_eq!(result, []);
/// ```
pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Return an empty vector if len is 0 or greater than the length of the input string
    if len == 0 || len > digits.len() {
        return Vec::new();
    }

    // Create a vector to store the resulting substrings
    let mut result = Vec::with_capacity(digits.len() - len + 1);

    // Iterate over the string, extracting substrings of the specified length
    for i in 0..=digits.len() - len {
        // Push a substring of length `len` to the result vector
        result.push(digits[i..i + len].to_string());
    }

    result
}