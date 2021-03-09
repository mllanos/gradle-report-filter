use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let query = &args[2];
    let mut i = 0;
    let mut matches = VecDeque::new();

    println!("Searching for [{}] in {}", query, filename);
    
    let file = BufReader::new(File::open(filename).unwrap());
    let mut lines: Vec<_> = file.lines().map(|line| { line.unwrap() }).collect();
    lines.reverse();

    for line in lines.clone() {
        if line.contains("+") || line.contains("\\") {
            if line.contains(query) {
                let mut level = line.matches("    ").count();
                matches.push_front(line);
                loop {
                    for _line in lines.clone().drain(i + 1..lines.len()) {        
                        let _level = _line.matches("    ").count();
                        if _level == level - 1 {
                            matches.push_front(_line);
                            level = _level;
                            if level == 0 {
                                break;
                            }
                        }
                    }
                    if level == 0 {
                        break;
                    }
                }
            }
        }
        i = i + 1;
    }

    for m in matches.iter() {
        println!("{}", m);
    }
}
