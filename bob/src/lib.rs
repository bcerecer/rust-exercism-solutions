pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if msg.len() == 0 => "Fine. Be that way!",
        msg if msg.ends_with("?") && is_yelling(msg) => "Calm down, I know what I'm doing!",
        msg if msg.ends_with("?") => "Sure.",
        msg if is_yelling(msg) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    let is_message_uppercase: bool = message.to_uppercase() == message;
    let have_letters: bool = message.chars().filter(|c| c.is_alphabetic()).count() > 0;
    is_message_uppercase && have_letters
}
