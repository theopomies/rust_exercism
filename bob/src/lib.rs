pub fn reply(message: &str) -> &str {
    let is_question = message.trim_end().ends_with('?');
    let is_yelled =
        message.contains(char::is_alphabetic) && message.to_ascii_uppercase() == message;
    let is_empty = message.trim().is_empty();

    match (is_question, is_yelled, is_empty) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Sure.",
        (_, true, _) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
