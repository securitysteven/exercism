pub fn reply(message: &str) -> &str {
    // "Sure." This is his response if you ask him a question, such as "How are you?" The convention used for questions is that it ends with a question mark.
    if message.ends_with('?') {"Sure"}
    // "Whoa, chill out!" This is his answer if you YELL AT HIM. The convention used for yelling is ALL CAPITAL LETTERS.
    if message.chars().all(|c| c.is_uppercase()) {"Whoa, chill out!"}
    // "Calm down, I know what I'm doing!" This is what he says if you yell a question at him.
    if message.chars().all(|c| c.is_uppercase()) && message.ends_with('?') {"Calm down, I know what I'm doing!"}
    // "Fine. Be that way!" This is how he responds to silence. The convention used for silence is nothing, or various combinations of whitespace characters.
    if message.is_empty() || message.chars().all(|c| c.is_whitespace()) {"Fine. Be that way!"}
    // "Whatever." This is what he answers to anything else.
    else "Whatever"
}
