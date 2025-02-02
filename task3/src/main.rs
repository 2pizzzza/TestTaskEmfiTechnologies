mod matrix;
mod tests;

use crate::matrix::{matrix_multiply, parse_file_to_matrix};
use std::io::BufRead;

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


