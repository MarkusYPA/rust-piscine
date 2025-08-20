const VOWELS: &str = "aeiouAEIOU";

pub fn pig_latin(text: &str) -> String {
    if VOWELS.chars().any(|c| c == text.chars().next().unwrap()) {
        // Starts with vowel: put ay at end
        return format!("{}ay", text);
    } else {
        if is_qu_word(text) {
            // three letters to end + ay
            return format!(
                "{}{}ay",
                text.chars().collect::<Vec<_>>()[3..]
                    .iter()
                    .collect::<String>(),
                text.chars().collect::<Vec<_>>()[..3]
                    .iter()
                    .collect::<String>()
            );
        } else {
            // cut at first vowel
            if let Some(first_vowel_index) = text.chars().position(|c| VOWELS.contains(c)) {
                return format!(
                    "{}{}ay",
                    text.chars().collect::<Vec<_>>()[first_vowel_index..]
                        .iter()
                        .collect::<String>(),
                    text.chars().collect::<Vec<_>>()[..first_vowel_index]
                        .iter()
                        .collect::<String>()
                );
            } else {
                // only consonants!
                return format!("{}ay", text);
            }
        }
    }
}

// are second and third letters q and u
fn is_qu_word(s: &str) -> bool {
    match s.find("qu") {
        Some(1) => true,
        _ => false,
    }
}
