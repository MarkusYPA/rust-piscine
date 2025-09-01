use std::cmp::min;

#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut vals = self.numbers.to_owned();
        vals.sort();
        vals.reverse();

        let mut result = Vec::new();
        let rng = min(3, vals.len());
        for i in 0..rng {
            result.push(vals[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests;
