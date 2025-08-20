pub fn number_logic(num: u32) -> bool {
    let mut my_num = num.clone();
    let p = num.to_string().len() as u32;
    let mut sum = 0;
    while my_num > 0 {
        let n = my_num % 10;
        my_num /= 10;
        sum += n.pow(p);
    }   

    sum == num
}

#[cfg(test)]
mod tests;