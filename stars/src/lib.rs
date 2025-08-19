pub fn stars(n: u32) -> String {
    let mut s = String::new();
    for _ in 0..2_i32.pow(n) {
        s.push('*');
    }
    s
}

#[cfg(test)]
mod tests;