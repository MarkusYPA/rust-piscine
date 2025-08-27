use super::*;

#[test]
fn zero_property() {
    let matrix: Matrix<u32> = Matrix::zero(3, 4);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::zero(2, 2);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(matrix, expected);
}

#[test]
fn identity_matrix() {
    let matrix: Matrix<u32> = Matrix::identity(2);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::identity(3);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(matrix, expected);
}