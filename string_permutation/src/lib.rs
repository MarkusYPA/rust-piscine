//use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    for c_0 in s1.chars() {
        let mut count_1 = 0;
        let mut count_2 = 0;

        for c_1 in s1.chars() {
            if c_1 == c_0 {
                count_1 += 1;
            }
        }

        for c_2 in s2.chars() {
            if c_2 == c_0 {
                count_2 += 1;
            }
        }

        if count_1 != count_2 {
            return false;
        }
    }

    true
}
