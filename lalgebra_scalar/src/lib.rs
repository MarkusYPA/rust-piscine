pub use std::ops::{Add, Div, Mul, Sub};                     // pub for matrix_ops

// Scalar is a trait that implements other traits: Add, Div, Mul and Sub
// This is trait inheritance (supertraits)
pub trait Scalar: Add + Div + Mul + Sub + Sized + Clone {   // Clone needed in 'matrix'
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = Self;
    fn one() -> Self::Item {
        1
    }

    fn zero() -> Self::Item {
        0
    }
}

impl Scalar for i32 {
    type Item = Self;
    fn one() -> Self::Item {
        1
    }

    fn zero() -> Self::Item {
        0
    }
}

impl Scalar for u64 {
    type Item = Self;
    fn one() -> Self::Item {
        1
    }

    fn zero() -> Self::Item {
        0
    }
}

impl Scalar for i64 {
    type Item = Self;
    fn one() -> Self::Item {
        1
    }

    fn zero() -> Self::Item {
        0
    }
}

impl Scalar for f32 {
    type Item = Self;
    fn one() -> Self::Item {
        1.0
    }

    fn zero() -> Self::Item {
        0.0
    }
}

impl Scalar for f64 {
    type Item = Self;
    fn one() -> Self::Item {
        1.0
    }

    fn zero() -> Self::Item {
        0.0
    }
}

#[cfg(test)]
mod tests;