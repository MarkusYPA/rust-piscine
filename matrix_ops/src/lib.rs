use lalgebra_scalar::*;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut nm = Matrix::new();

        for i in 0..n {
            let mut row = vec![T::zero(); n];
            row[i] = T::one();
            nm.0.push(row);
        }

        nm
    }
}

impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T> + Add<Output = T>,
{
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.0.len() != self.0.len() {
            return None;
        }
        let mut result = Matrix::new();

        for (i, row) in self.0.iter().enumerate() {
            if rhs.0[i].len() != row.len() {
                return None;
            }

            let mut result_row = Vec::new();
            for (j, col) in row.iter().enumerate() {
                result_row.push(col.clone() + rhs.0[i][j].clone());
            }

            result.0.push(result_row);
        }

        Some(result)
    }
}

impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T> + Sub<Output = T>,
{
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0.len() != self.0.len() {
            return None;
        }
        let mut result = Matrix::new();

        for (i, row) in self.0.iter().enumerate() {
            if rhs.0[i].len() != row.len() {
                return None;
            }

            let mut result_row = Vec::new();
            for (j, col) in row.iter().enumerate() {
                result_row.push(col.clone() - rhs.0[i][j].clone());
            }

            result.0.push(result_row);
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests;
