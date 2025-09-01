use std::collections::HashMap;

// Lifetimes first, types second
pub fn slices_to_map<'a, T, U>(ks: &'a [T], vs: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    /* let mut result = HashMap::new();
    let pairs = ks.iter().zip(vs.iter());
    for (k, v) in pairs {
        result.insert(k, v);
    }
    result */

    // Better: collect() turns tuple pairs to hashmap automatically 
    ks.iter().zip(vs.iter()).collect()
}

#[cfg(test)]
mod tests;
