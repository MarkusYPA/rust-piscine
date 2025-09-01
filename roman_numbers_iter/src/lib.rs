mod roman;

pub use roman::{RomanDigit, RomanNumber};

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            return None;
        }

        // get num value of current, add 1, translate to roman, return
        Some(RomanNumber::from(u32::from(self.clone()) + 1))
    }
}

#[cfg(test)]
mod tests;
