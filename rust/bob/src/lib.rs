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
    let msg = message.trim_end();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = msg.ends_with('?');
    let is_yelling =
        msg.chars().any(|ch| ch.is_alphabetic()) && msg == msg.to_uppercase();

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}