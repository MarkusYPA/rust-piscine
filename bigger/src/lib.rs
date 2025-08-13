use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut b = h.values().next().unwrap();

    for v in h.values() {
        if v > b {
            b = v;
        }
    }

    b.to_owned()
}