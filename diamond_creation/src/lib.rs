pub fn get_diamond(c: char) -> Vec<String> {
    let side: u8 = ((c as u8) - ('A' as u8)) * 2 + 1;
    let midpoint = &side / 2;

    let mut diamond = Vec::new();
    let mut current = 'A';

    for row in 0..side {
        diamond.push(String::new());
        for col in 0..side {
            if col == (midpoint - (current as u8 - b'A'))
                || col == (midpoint + (current as u8 - b'A'))
            {
                diamond[row as usize].push(current);
            } else {
                diamond[row as usize].push(' ');
            }
        }

        if row < midpoint {
            current = (current as u8 + 1) as char;
        } else {
            current = (current as u8 - 1) as char;
        }
    }

    diamond
}

#[cfg(test)]
mod tests;