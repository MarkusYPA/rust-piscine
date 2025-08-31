use std::ops::Mul;
use lalgebra_scalar::*;

#[derive(Debug, PartialEq, Clone)]
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

impl<T> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        todo!()
    }

    pub fn number_of_rows(&self) -> usize {
        todo!()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        todo!()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        todo!()
    }
}

impl<T> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests;
