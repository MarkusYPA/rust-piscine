pub fn talking(text: &str) -> &str {
    let text = text.trim();

    if text.is_empty() {
        return "Just say something!";
    }

    let yell = text
        .chars()
        .all(|c| !c.is_alphabetic() || c.is_ascii_uppercase())
        && text.chars().any(|c| c.is_alphabetic());
    let ask = text.chars().last().unwrap() == '?';

    if yell && ask {
        return "Quiet, I am thinking!";
    }

    if yell {
        return "There is no need to yell, calm down!";
    }

    if ask {
        return "Sure.";
    }

    "Interesting"
}
#[cfg(test)]
mod tests;
