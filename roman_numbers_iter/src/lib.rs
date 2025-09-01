mod roman;

pub use roman::{RomanDigit, RomanNumber};

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {

        

        todo!()
    }
}

#[cfg(test)]
mod tests;