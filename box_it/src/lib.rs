pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result: Vec<Box<u32>> = Vec::new();
    s.split_ascii_whitespace().for_each(|n| {
        if n.ends_with("k") {
            let val = n.strip_suffix("k").unwrap().parse::<f64>().unwrap() * 1000.0;
            result.push(Box::new(val.round() as u32));
        } else {
            let val = n.parse::<f64>().unwrap();
            result.push(Box::new(val.round() as u32));
        }
    });
    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.iter().map(|b| *b.as_ref()).collect()
}

#[cfg(test)]
mod tests;
