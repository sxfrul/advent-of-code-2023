use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
    .expect("Should have been able to read the file");

    let line_split: Vec<&str> = input.lines().collect();
    let mut input_hm = HashMap::new();

    for line in line_split {
        let handbid_split: Vec<&str> = line.split(" ").collect();

        // storing key value pair
        let key = format!("{}", handbid_split[0]);
        let value = format!("{}", handbid_split[1]);
        input_hm.insert(key, value);
    }
    println!("{:?}", input_hm);
}