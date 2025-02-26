/// Determines the appropriate response based on the given message.
///
/// # Arguments
///
/// * `message` - A string slice (`&str`) representing the input message.
///
/// # Returns
///
/// * A string slice (`&str`) containing Bob's response.
///
/// # Behavior:
/// - If the message is **empty** or whitespace → Returns `"Fine. Be that way!"`
/// - If the message is **yelling (all uppercase)** AND a **question** → Returns `"Calm down, I know what I'm doing!"`
/// - If the message is **yelling (all uppercase)** → Returns `"Whoa, chill out!"`
/// - If the message is a **question** (ends with `?`) → Returns `"Sure."`
/// - Otherwise, returns `"Whatever."`
pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    match (
        msg.ends_with('?'),
        msg.chars().any(|c| c.is_alphabetic()),
        msg.chars().all(|c| !c.is_lowercase()), // Efficient yelling check
    ) {
        (_, _, true) if msg.is_empty() => "Fine. Be that way!", // Handles empty messages
        (true, true, true) => "Calm down, I know what I'm doing!", // Yelling question
        (false, true, true) => "Whoa, chill out!", // Yelling statement
        (true, _, _) => "Sure.", // Regular question
        _ => "Whatever.", // Default case
    }
}