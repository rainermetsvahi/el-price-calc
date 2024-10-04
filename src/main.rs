use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No input file given");
        process::exit(-2);
    }
    let file_path = &args[1];

    println!("Reading file {file_path}");

    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
