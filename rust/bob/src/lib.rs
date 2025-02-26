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
    let trimmed = message.trim(); // Remove leading & trailing whitespace

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && trimmed.chars().all(|c| !c.is_lowercase());

    match (is_yelling, trimmed.ends_with('?')) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}