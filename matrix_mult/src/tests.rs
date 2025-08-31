    use super::*;
    use lib::{Kind, TestProperties};

    #[test]
    fn get_row() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "row",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], vec![3u32, 6], matrix.row(0));
        test.assert_with_message(&[Box::new(matrix.clone())], vec![8u32, 0], matrix.row(1));
    }

    #[test]
    fn get_col() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "col",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.col(0), vec![3u32, 8]);
        test.assert_with_message(&[Box::new(matrix.clone())], vec![6u32, 0], matrix.col(1));
    }

    #[test]
    fn matrix_multiplication() {
        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 0]]);
        let test = TestProperties {
            name: "*",
            kind: Kind::Operator,
        };
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            Some(expected),
        );

        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 1]]);
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            None,
        );
    }