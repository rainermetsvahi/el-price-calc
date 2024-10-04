use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

// CSV field separator
const SEPARATOR: &str = ";";

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

    let mut sum: f64 = 0.0;
    let mut current_line: i32 = 1;

    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(ip) = line {
                    let parts: Vec<&str> = ip.split(SEPARATOR).collect();
                    if parts.len() == 3 {
                        let kwh: f64 = read_float(parts[1], current_line);
                        let price: f64 = read_float_or_default_if_empty(parts[2], current_line, 0.0);
                        let cost = calc_cost(kwh, price);
                        sum += cost;
                    } else {
                        println!("Skipped line {current_line}. Unexpected format");
                    }
                }
                current_line = current_line + 1;
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
    let read_lines = current_line - 1;
    println!("Read {read_lines} lines, sum is {sum:.2}")
}

fn calc_cost(kwh: f64, price_per_kwh: f64) -> f64 {
    if price_per_kwh < 0.0 {
        return 0.0;
    }
    return kwh * price_per_kwh;
}

fn read_float_or_default_if_empty(
    value: &str,
    line_number: i32,
    default_value: f64) -> f64
{
    if value.trim().is_empty() {
        return default_value;
    }
    return read_float(value, line_number);
}

fn read_float(value: &str, line_number: i32) -> f64 {
    let result: f64 = value.trim()
        .replace(',', ".")
        .parse()
        .expect(&format!("Invalid number: {} on line {}", value, line_number));
    return result;
}
