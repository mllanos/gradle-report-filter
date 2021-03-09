use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let query = &args[2];

    println!("Searching for [{}] in {}", query, filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if l.contains(query) {
                    println!("{}", l);
                }
            }
        }    
    }
}
