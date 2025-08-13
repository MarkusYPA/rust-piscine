use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut b = h.values().next().unwrap();
    for v in h.values() {
        if v > b {
            b = v;
        }
    }
    b.to_owned()

    // All of these work too:
    //h.values().max().unwrap().to_owned()
    //*h.values().max().unwrap()
    //h.into_values().max().unwrap()
}