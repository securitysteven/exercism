pub fn reply(message: &str) -> &str {
    let trimmed = message.trim(); // Remove leading and trailing whitespace

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if trimmed.chars().all(|c| c.is_alphabetic() && c.is_uppercase()) && trimmed.ends_with('?') {
        "Calm down, I know what I'm doing!"
    } else if trimmed.chars().any(|c| c.is_alphabetic()) && trimmed.chars().all(|c| !c.is_lowercase()) {
        "Whoa, chill out!"
    } else if trimmed.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
