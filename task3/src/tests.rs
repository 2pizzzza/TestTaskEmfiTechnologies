#[cfg(test)]
mod tests {
    use crate::matrix::{matrix_multiply};

    #[test]
    fn test_matrix_multiplication() {
        let matrix_a = vec![vec![1, 2], vec![3, 4]];
        let matrix_b = vec![vec![5, 6], vec![7, 8]];
        let expected = vec![vec![19, 22], vec![43, 50]];
        let result = matrix_multiply(&matrix_a, &matrix_b).unwrap();
        assert_eq!(result, expected);
    }
}
