use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    (list.iter().sum::<i32>() as f64) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut my_list = list.iter().collect::<Vec<_>>();

    my_list.sort();

    if my_list.len() % 2 == 1 {
        *my_list[my_list.len() / 2]
    } else {
        (my_list[(my_list.len() - 1) / 2] + my_list[my_list.len() / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freqs = HashMap::new();

    for v in list {
        let count = freqs.entry(v).or_insert(0);
        *count += 1;
    }

    **freqs.iter().max_by_key(|k| k.1).unwrap().0

    /* let most = *freqs.values().max().unwrap();
    for (k, v) in freqs {
        if v == most {
            return *k;
        }
    }

    return 0; */
}
