pub fn reply(message: &str) -> &str {
    let trimmed = message.trim(); // Remove leading & trailing whitespace

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let has_letters = trimmed.chars().any(|c| c.is_alphabetic()); // At least one letter
    let is_yelling = has_letters && trimmed.chars().all(|c| !c.is_lowercase()); // No lowercase letters

    match (is_yelling, trimmed.ends_with('?')) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}