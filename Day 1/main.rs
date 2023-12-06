use std::fs;

fn main() {
    // --snip--
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let splitted_input: Vec<&str> = input.lines().collect();

    let mut value = 0;

    for string in splitted_input { 
        let character: Vec<char> = string.chars().collect::<Vec<_>>();
        let mut vector2: Vec<char> =  Vec::new();
        for element in character {
            if element.is_numeric() {
                let mut vector3: Vec<char> = vec![element];
                vector2.append(&mut vector3);
            }
        };
        let reverse: Vec<char> = vector2.iter().copied().rev().collect(); //reading vector in reverse
        let concatenated_string = format!("{}{}", vector2[0], reverse[0]); //concatenating first of non reverse + reverse
        println!("{}", concatenated_string);
        let num = concatenated_string.parse::<i32>().unwrap();
        value += num;
        
    }
    println!("Result is: {}", value);
}