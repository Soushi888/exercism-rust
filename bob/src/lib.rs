pub fn reply(message: &str) -> &str {
    let response;
    let mut type_of_message: &str = "";

    if message.to_uppercase() == message { type_of_message = "yelling" }
    if message.trim() == "" { type_of_message = "empty" }
    if message.trim().len() != 0 {
        if message.trim().chars().nth(message.trim().len() - 1).unwrap() == '?' { type_of_message = "question" }
        if message.to_uppercase() == message && message.trim().chars().nth(message.trim().len() - 1).unwrap() == '?' { type_of_message = "yelled question" }
    }

    println!("{}", message.trim());

    match type_of_message {
        "question" => { response = "Sure." }
        "yelling" => { response = "Whoa, chill out!" }
        "yelled question" => { response = "Calm down, I know what I'm doing!" }
        "empty" => { response = "Fine. Be that way!" }
        _ => { response = "Whatever."; }
    }

    response
}
