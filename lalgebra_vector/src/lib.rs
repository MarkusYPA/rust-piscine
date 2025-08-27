use std::ops::{Add, Mul};
use std::iter::Sum;

use lalgebra_scalar::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T> Add for Vector<T>
where
    T: Scalar,          // Specify Item? T: Scalar<Item = T>,
    T: Add<Output = T>, // Necessary to push values to result.0 vector
{
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut result = Vector(Vec::new()); // no need to define Vec type
        for i in 0..self.0.len() {
            result.0.push(self.0[i].clone() + rhs.0[i].clone());
        }
        Some(result)
    }
}

impl<T> Vector<T>
where
    T: Scalar + Sum + Mul<Output = T>,
{
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        /* let mut result = T::zero();
        for i in 0..self.0.len() {
                result += self.0[i].clone() * other.0[i].clone();
        }
        Some(result) */

        Some(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.clone() * y.clone())
                .sum(),
        )
    }
}

//#[cfg(test)]
//mod tests;
