use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    match process_file(path) {
        Ok((lines , words, chars))=> {
            println!("Lines: {lines} \nWords: {words} \nChars:{chars}\n")
        }
        Err(e)=> eprintln!("Error: {e}")
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


