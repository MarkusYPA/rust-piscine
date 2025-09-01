pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return vec![];
    }

    arr.iter()
        .enumerate()
        .map(|(i, _)| {
            arr.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, v)| v)
                .product()  // "empty iterator returns the one value", therefore the exception at the beginning
        })
        .collect()
}

#[cfg(test)]
mod tests;
