pub fn capitalize_first(input: &str) -> String {
    format!(
        "{}{}",
        input.chars().next().unwrap().to_ascii_uppercase(),
        &input[1..]
    )
}

pub fn title_case(input: &str) -> String {
    input
        .split(' ')
        .map(|w| capitalize_first(w))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                return c.to_ascii_uppercase();
            } else if c.is_ascii_uppercase() {
                return c.to_ascii_lowercase();
            } else {
                return c;
            }
        })
        .collect()
}
