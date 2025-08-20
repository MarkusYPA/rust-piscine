pub fn scytale_cipher(message: &str, sz: usize) -> String {
    //let rows_n = (message.len() as f64 / sz as f64).ceil() as usize;
    let rows_n = message.len().div_ceil(sz);
    let empty_row = vec![' '; sz];
    let mut rows = vec![empty_row; rows_n];

    let mut row_ind = 0;
    for (i, c) in message.chars().enumerate() {
        rows[row_ind][i % sz] = c;
        if (i + 1) % sz == 0 {
            row_ind += 1;
        }
    }

    let mut result = String::new();
    for i in 0..sz {
        for row in &rows {
            result.push(row[i]);
        }
    }

    result.trim().to_owned()
}

/* use std::iter;

pub fn scytale_cipher(message: &str, i: usize) -> String {
    let i = i.min(message.len());
    if message.is_empty() || i == 0 {
        String::new()
    } else {
        let cols = message.len().div_ceil(i);

        (0..i)
            .map(|n| {
                message
                    .chars()
                    .skip(n)
                    .step_by(i)
                    .chain(iter::repeat_n(' ', cols - (message.len() - n).div_ceil(i)))
                    .collect::<String>()
            })
            .collect::<String>()
            .trim_end()
            .to_owned()
    }
} */

#[cfg(test)]
mod tests;
