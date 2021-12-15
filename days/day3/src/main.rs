use std::{fs, process::exit};

fn main() {
    let bits_vec = get_bits(&parse_input(&load_file_as_text("input.txt")))
        .iter()
        .map(|e| e.to_vec())
        .collect::<Vec<Vec<bool>>>();

    let gamma_bits = [false; 12]
        .iter()
        .enumerate()
        .map(|(i, _)| most_common_bit_in_column(&bits_vec, i as i64))
        .collect::<Vec<bool>>();

    let epsilon_bits = gamma_bits.iter().map(|e| !e).collect::<Vec<bool>>();

    let gamma = bin_to_num(&gamma_bits);
    let epsilon = bin_to_num(&epsilon_bits);

    let power_consumption = gamma * epsilon;

    print!("{:?}", power_consumption)
}

fn bin_to_num(bits: &Vec<bool>) -> u64 {
    let mut num = 0;

    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit {
            num += 2_u64.pow(i as u32)
        }
    }

    return num;
}

fn most_common_bit_in_column(bits_list: &Vec<Vec<bool>>, column: i64) -> bool {
    let mut sum: u64 = 0;

    for bits in bits_list {
        sum += bits[column as usize] as u64;
    }

    return sum > (bits_list.len() as u64) / 2;
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

    return parsed_lines
        .iter()
        .cloned()
        .filter(|s| s.len() != 0)
        .collect::<Vec<String>>();
}

fn get_bits(lines: &Vec<String>) -> Vec<[bool; 12]> {
    let mut bits_list: Vec<[bool; 12]> = Vec::new();

    for line in lines {
        let digits = line
            .trim()
            .split("")
            .map(str::to_string)
            .filter(|s| s.len() != 0)
            .collect::<Vec<String>>();

        let mut bits: [bool; 12] = [false; 12];

        for (i, digit) in digits.iter().enumerate() {
            if i > 11 {
                println!("more digits than expected! ({})", digit);
                break;
            }

            let bit = match digit.as_str() {
                "0" => false,
                "1" => true,
                "" => continue,
                _ => {
                    println!("Reading bits went wrong");
                    exit(1)
                }
            };

            bits[i] = bit;
        }

        bits_list.push(bits)
    }

    return bits_list;
}
