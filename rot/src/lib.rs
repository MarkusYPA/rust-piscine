pub fn rotate(input: &str, key: i8) -> String {
    let alphas = "abcdefghijklmnopqrstuvwxyz";

    let mut result = String::new();
    for c in input.chars() {
        if let Some(i) = alphas
            .chars()
            .position(|alpha| c.to_ascii_lowercase() == alpha)
        {
            let is_up = c.is_ascii_uppercase();

            let mut new_i = (i as i8 + key) % alphas.len() as i8;
            if new_i < 0 {
                new_i = new_i + alphas.len() as i8;
            }

            if is_up {
                result.push(
                    alphas.chars().collect::<Vec<char>>()[new_i as usize].to_ascii_uppercase(),
                );
            } else {
                result.push(alphas.chars().collect::<Vec<char>>()[new_i as usize]);
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests;
