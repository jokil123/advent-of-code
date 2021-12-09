use std::{fs, process::exit};

fn main() {
    println!("Hello, world!");
    println!("{:?}", get_bits(&parse_input(&load_file_as_text("input.txt"))));
}

fn load_file_as_text(rel: &str) -> String {
    return fs::read_to_string(rel).expect("Something went wrong reading the file");
}

fn parse_input(input: &String) -> Vec<String> {
    let mut parsed_lines: Vec<String> = Vec::new();
    let lines: Vec<String> = input.split("\n").map(str::to_string).collect();

    for line in lines {
        parsed_lines.push(line.trim().to_string());
    }

    return parsed_lines;
}

fn get_bits(lines: &Vec<String>) -> Vec<[bool; 12]> {
    let bits_list: Vec<[bool; 12]> = Vec::new();

    for line in lines {
        let digits: Vec<String> = line.split("").map(str::to_string).collect();
        let mut bits: [bool; 12] = [false; 12];

        for (i, digit) in digits.iter().enumerate() {

            if i > 11 {
                break;
            }

            let bit = match digit.as_str() {
                "0" => false,
                "1" => true,
                "" => continue,
                _ => {println!("Reading bits went wrong"); exit(1)}
            };

            bits[i] = bit;
            println!("{:?}", bits)
        }
    }

    return bits_list
}
