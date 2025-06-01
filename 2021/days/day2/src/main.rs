use std::fs;

fn main() {
    let pos = run_instructions(parse_instructions(parse_input(&load_file_as_text(
        "input.txt",
    ))));

    println!("{:?}", pos);

    println!("{:?}", pos.0 * pos.1);
}

#[derive(Debug)]
enum InstructionTypes {
    UP,
    DOWN,
    FORWARD,
}

#[derive(Debug)]
struct Instruction {
    instruction_name: InstructionTypes,
    amount: i64,
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

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let instruction_chunks: Vec<String> = line.split(" ").map(str::to_string).collect();

        let name = &instruction_chunks[0];
        let amount = &instruction_chunks[1];

        let instruction: Instruction = Instruction {
            instruction_name: match name.to_uppercase().as_str() {
                "UP" => InstructionTypes::UP,
                "DOWN" => InstructionTypes::DOWN,
                "FORWARD" => InstructionTypes::FORWARD,
                _ => {
                    println!("Something went wrong!");
                    break;
                }
            },
            amount: match amount.parse::<i64>() {
                Ok(num) => num,
                Err(_) => break,
            },
        };

        println!("{:?}", instruction);

        instructions.push(instruction);
    }

    println!("{}", instructions.len());

    return instructions;
}

fn run_instructions(instructions: Vec<Instruction>) -> (i64, i64) {
    let mut aim: i64 = 0;
    let mut position: (i64, i64) = (0, 0);
    for instruction in instructions {
        match instruction.instruction_name {
            InstructionTypes::UP => {
                aim -= instruction.amount;
                // position.1 -= instruction.amount;
            }
            InstructionTypes::DOWN => {
                aim += instruction.amount;
                // position.1 += instruction.amount;
            }
            InstructionTypes::FORWARD => {
                // position.0 += instruction.amount;
                position.0 += instruction.amount;
                position.1 += instruction.amount * aim;
            }
        }
    }

    return position;
}
