//use std::ops::{AddAssign, Sub};

pub struct StepIterator<T> {
    beg: T,
    end: T,
    started: bool,
    step: T,
}

pub use std::ops::Add;

impl<T> StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd + PartialEq,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            end,
            started: false,
            step,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: PartialEq + Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end || self.beg + self.step > self.end {
            return None;
        }

        // first time 'round return 'beginning'
        if !self.started {
            self.started = true;
            return Some(self.beg);
        }

        // store current value to 'beginning'
        // won't be able to reset it like this
        self.beg = self.beg + self.step;

        Some(self.beg)
    }
}

/* impl<T> std::fmt::Display for StepIterator<T>
where
    T: std::ops::AddAssign,
    T: Clone,
    T: PartialOrd,
    T: std::ops::Sub,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let vals = self.next();

        write!(f, "{}", self.beg)
        //Ok(())
    }
} */

#[cfg(test)]
mod tests;
