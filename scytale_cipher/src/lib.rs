pub fn scytale_cipher(message: &str, sz: usize) -> String {
    let rows_n = (message.len() as f64 / sz as f64).ceil() as usize;
    let empty_row = vec![' '; sz];
    let mut rows = vec![empty_row; rows_n];

    let mut row_ind = 0;
    for (i, c) in message.chars().enumerate() {
        //println!("row {} ind {}", row_ind, i);
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

    //println!("rows: {:?}", rows);

    result.trim().to_owned()
}

#[cfg(test)]
mod tests;
