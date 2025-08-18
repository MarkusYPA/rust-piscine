#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let alpha_low = "abcdefghijklmnopqrstuvwxyz".to_owned();
    let rev_alpha_low: String = alpha_low.chars().rev().collect();
    let alpha_up = alpha_low.to_ascii_uppercase();
    let rev_alpha_up: String = rev_alpha_low.to_ascii_uppercase();

    let mut correct = String::new();
    for c in original.chars() {
        // get index of low char
        if let Some(low_i) = alpha_low.chars().position(|x| x == c) {
            correct.push(rev_alpha_low.chars().nth(low_i).unwrap());
        } else if let Some(up_i) = alpha_up.chars().position(|x| x == c) {
            correct.push(rev_alpha_up.chars().nth(up_i).unwrap());
        } else {
            correct.push(c);
        }
    }

    match correct == ciphered.to_owned() {
        true => Ok(()),
        false => Err(CipherError { expected: correct }),
    }
}
