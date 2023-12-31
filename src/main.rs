use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Where is my console output!");
    let input_path = Path::new("resources/day1/input");
    let input = fs::read_to_string(input_path)
        .expect("should have read the file");
    let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let mut results: Vec<u32> = Vec::with_capacity(1000);
    for (_, line) in lines.into_iter().enumerate() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for character in line.chars() {
            if character.is_numeric() {
                if first.is_none() {
                    first = character.to_digit(10);
                }
                last = character.to_digit(10);        
            }
        }
        let result = first.unwrap() * 10 + last.unwrap();
        results.push(result);
    }
    let output_path = Path::new("resources/day1/output");
    let mut output_file = File::create(output_path).expect("should have opened file");
    for number in results.as_slice() {
        writeln!(output_file, "{}", number.to_string()).expect("should write to file");
    }
    let total: u32 = results.into_iter().sum();
    println!("{total}");
}

fn day1solver() { 
    println!("Where is my console output!");
    let input_path = Path::new("resources/day1/input");
    let input = fs::read_to_string(input_path)
        .expect("should have read the file");
    let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let mut results: Vec<u32> = Vec::with_capacity(1000);
    for (_, line) in lines.into_iter().enumerate() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for character in line.chars() {
            if character.is_numeric() {
                if first.is_none() {
                    first = character.to_digit(10);
                }
                last = character.to_digit(10);        
            }
        }
        let result = first.unwrap() * 10 + last.unwrap();
        results.push(result);
    }
    let output_path = Path::new("resources/day1/output");
    let mut output_file = File::create(output_path).expect("should have opened file");
    for number in results.as_slice() {
        writeln!(output_file, "{}", number.to_string()).expect("should write to file");
    }
    let total: u32 = results.into_iter().sum();
    println!("{total}");
}
