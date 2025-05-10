pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    let is_yelling = trimmed_message.to_uppercase() == trimmed_message;
    let contains_a_letter = trimmed_message.chars().any(|x| x.is_alphabetic());
    let is_a_question = trimmed_message.chars().last().unwrap_or_default() == '?';
    let is_empty = trimmed_message.is_empty();

    if is_yelling && contains_a_letter {
        if is_a_question {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if is_a_question {
        "Sure."
    } else if is_empty {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
