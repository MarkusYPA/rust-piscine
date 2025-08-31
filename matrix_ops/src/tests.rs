use super::*;

#[test]
fn addition() {
    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
    assert_eq!(matrix + matrix_2, Some(expected));

    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    assert_eq!(matrix + matrix_2, None);
}

#[test]
fn subtraction() {
    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(matrix - matrix_2, Some(expected));

    let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
    let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
    assert_eq!(matrix - matrix_2, None);
}
