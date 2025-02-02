use std::io;
use std::env;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify the file that needs to be analyzed, or use 'cargo run help' for usage instructions.");
        std::process::exit(1);
    }

    if args[1] == "help" {
        println!("Usage: cargo run <file>\nThis program analyzes the content of the specified file and prints out the number of lines, words, and characters.");
        std::process::exit(1);
    }

    let path = &args[1];
    match process_file(path) {
        Ok((lines, words, chars)) => {
            if lines == 0 && words == 0 && chars == 0 {
                println!("Entered file is empty :((")
            } else {
                println!("File analysis results:\n- Lines: {lines}\n- Words: {words}\n- Characters: {chars}");
            }
        }
        Err(e) => eprintln!("Error reading the file: {e}")
    }
}


fn process_file(path: &str) -> Result<(usize, usize, usize), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut words = 0;
    let mut lines = 0;
    let mut chars = 0;

    for line in reader.lines() {
        let line = line?;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
    }

    Ok((words, lines, chars))
}




