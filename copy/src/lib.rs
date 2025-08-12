pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    (
        a.to_owned(),
        a.split(' ')
            .map(|num| num.parse::<f64>().unwrap().exp().to_string())
            .collect(),
    )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b.iter().map(|val| (*val as f64).abs().ln()).collect();

    /*
    let mut res = Vec::new();
    for num in b.clone() {
        res.push((num as f64).abs().ln());
    }
     */

    (b, res)
}
