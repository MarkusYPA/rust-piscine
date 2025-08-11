pub fn factorial(num: u64) -> u64 {
    let mut res = 1;
    for n in 2..num+1 {
        res *= n;
    }
    res
}