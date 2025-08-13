use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hm = HashMap::new();

    for w in words {
        let count = hm.entry(*w).or_insert(0);
        *count += 1;
    }

    hm
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}