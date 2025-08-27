#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::{Add, Sub};

impl<T> Add for ThreeDVector<T> // no curly brace yet
where
    T: Add<Output = T>,     // require T implement Add so that it outputs T 
{
    type Output = ThreeDVector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        ThreeDVector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl <T>Sub for ThreeDVector<T>
where 
    T: Sub<Output = T>  // no beaks around T

{
    type Output = ThreeDVector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        ThreeDVector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

#[cfg(test)]
mod tests;