use std::sync::{Arc, Mutex};
use std::{io, thread};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matrix_a = parse_file_to_matrix("matrix_a.txt").expect("Failed to parse matrix_a");
    let matrix_b = parse_file_to_matrix("matrix_b.txt").expect("Failed to parse matrix_b");

    match matrix_multiply(&matrix_a, &matrix_b) {
        Ok(result) => {
            println!("Result:");
            for row in result {
                println!("{:?}", row);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

fn parse_file_to_matrix(file_path: &str) -> Result<Vec<Vec<i32>>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        matrix.push(row);
    }

    Ok(matrix)
}

fn matrix_multiply(matrix_a: &Vec<Vec<i32>>, matrix_b: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, String> {
    let rows_a = matrix_a.len();
    let cols_a = matrix_a[0].len();
    let rows_b = matrix_b.len();
    let cols_b = matrix_b[0].len();

    if cols_a != rows_b {
        return Err(format!("Columns of matrix_a ({cols_a}) must equal rows of matrix_b ({rows_b})"));
    }

    let result = vec![vec![0; cols_b]; rows_a];
    let result = Arc::new(Mutex::new(result));

    let mut handles = vec![];

    for i in 0..rows_a {
        let matrix_a = matrix_a.clone();
        let matrix_b = matrix_b.clone();
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            for j in 0..cols_b {
                let mut sum = 0;
                for k in 0..cols_a {
                    sum += matrix_a[i][k] * matrix_b[k][j];
                }
                let mut result = result.lock().unwrap();
                result[i][j] = sum;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let matrix_a = vec![vec![1, 2], vec![3, 4]];
        let matrix_b = vec![vec![5, 6], vec![7, 8]];
        let expected = vec![vec![19, 22], vec![43, 50]];
        let result = matrix_multiply(&matrix_a, &matrix_b).unwrap();
        assert_eq!(result, expected);
    }
}
