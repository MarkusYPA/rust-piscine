pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() {
        return Err("ERROR: illegal");
    } else {
        match message.to_ascii_lowercase().contains("stupid") {
            true => Err("ERROR: illegal"),
            false => Ok(message),
        }
    }
}