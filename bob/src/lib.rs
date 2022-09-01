pub fn reply(message: &str) -> &str {
    let response;
    let mut type_of_message: &str = "";
    let message = message.trim();

    let is_letters = message
        .replace("?", "")
        .chars().any(|c| c.is_alphabetic());

    if message.to_uppercase() == message && is_letters { type_of_message = "yelling" }
    if message == "" { type_of_message = "empty" }
    if message.len() != 0 {
        if message.chars().nth(message.len() - 1).unwrap() == '?' { type_of_message = "question" }

        if message.to_uppercase() == message && message.chars().nth(message.len() - 1).unwrap() == '?' && is_letters { type_of_message = "yelled question" }
    }

    match type_of_message {
        "question" => { response = "Sure." }
        "yelling" => { response = "Whoa, chill out!" }
        "yelled question" => { response = "Calm down, I know what I'm doing!" }
        "empty" => { response = "Fine. Be that way!" }
        _ => { response = "Whatever." }
    }

    response
}
