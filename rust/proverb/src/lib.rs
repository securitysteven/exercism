/// Builds a proverb-like string based on the items in the list.
///
/// The proverb starts with each item in the list being linked in a chain of consequences,
/// and ends with "And all for the want of a {}."
///
/// # Arguments
///
/// * `list` - A slice of strings, where each string represents an item in the proverb.
///
/// # Returns
///
/// A string representing the full proverb, constructed from the items in the list.
///
/// # Example
///
/// ```rust
/// let items = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];
/// let proverb = build_proverb(&items);
/// println!("{}", proverb);
/// ```
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new(); // Return an empty string for an empty list
    }

    let mut proverb = String::new();

    // Iterate over the list, constructing the proverb
    for i in 0..list.len() - 1 {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]));
    }

    // Handle the last item in the list
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}