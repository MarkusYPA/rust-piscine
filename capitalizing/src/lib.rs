pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    format!(
        "{}{}",
        input.chars().next().unwrap().to_ascii_uppercase(),
        &input[1..]
    )
}

pub fn title_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    /*     input
    .split(' ')
    .map(|w| capitalize_first(w))
    .collect::<Vec<String>>()
    .join(" ") */

    let mut is_first = true;
    let mut res = String::new();

    for c in input.chars() {
        if is_first && !c.is_whitespace() {
            res.push(c.to_ascii_uppercase());
            is_first = false;
        } else {
            if c.is_whitespace() {
                is_first = true;
            }
            res.push(c);
        }
    }
    res
}

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

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
