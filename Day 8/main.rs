use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to convert input -> string");

    let mut line_split: Vec<&str> = input.lines().collect();
    println!("{:?}", line_split);
    
    // instruction set
    let instruction = format!("{}", line_split[0]);

    line_split.remove(line_split.iter().position(|x| *x == instruction).expect("Instruction not found"));
    println!("{:?}", line_split);
    println!("{instruction}");
}