use lalgebra_scalar::*;
use std::iter::Sum;
use std::ops::Mul;

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

impl<T> Matrix<T>
where
    T: Scalar + Clone,
{
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T> + Mul<Output = T> + Sum, // Item = T necessary to create initial 'result' (Matrix::zero).  Mul<Output = T> and Sum necessary for .map() and .sum()
{
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        // self col values will be multiplied by rhs row values, must be same amount
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        // result: res.rows.len = self.rows.len, res.cols.len = rhs.cols.len
        let mut result = Matrix::zero(self.number_of_rows(), rhs.number_of_cols());

        // result row i: multiply self row i by each rhs column j
        for i in 0..result.number_of_rows() {
            for j in 0..result.number_of_cols() {
                result.0[i][j] = self
                    .row(i)
                    .iter()
                    .zip(rhs.col(j).iter()) // Combine two iterators into tuple pairs
                    .map(|(x, y)| x.clone() * y.clone()) // Tuple pairs into result of multiplication
                    .sum(); // sum of multiplications to result.0[i][j]
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests;
