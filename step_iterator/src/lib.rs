
#[allow(dead_code)]
pub struct StepIterator<T> {
    beg: T,     // not used here, but stores the original starting point
    end: T,
    current: T,
    step: T,
}

pub use std::ops::Add;

impl<T> StepIterator<T>
where 
    T: Copy,
 {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            end,
            current: beg,
            step,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let old_value = self.current;
        self.current = self.current + self.step;        
        Some(old_value)
    }
}

#[cfg(test)]
mod tests;
