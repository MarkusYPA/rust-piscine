use lalgebra_scalar::*;

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

#[cfg(test)]
mod tests;
