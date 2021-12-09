fn main() {
    println!("Hello, world!");
}

enum InstructionTypes {
    UP,
    DOWN,
    FORWARD,
}

struct Instruction {
    instruction_name: InstructionTypes,
    amount: i64,
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let instruction_chunks: Vec<String> = line.split("something").map(str::to_string).collect();

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

        instructions.push(Instruction {
            amount: 10,
            instruction_name: InstructionTypes::UP,
        });
    }

    return instructions;
}

fn run_instructions(instructions: Vec<Instruction>) -> (i64, i64) {
    let mut position: (i64, i64) = (0, 0);
    for instruction in instructions {
        match instruction.instruction_name {
            UP => {
                position.1 += instruction.amount;
            }
            DOWN => {
                position.1 -= instruction.amount;
            }
            FORWARD => {
                position.0 += instruction.amount;
            }
        }
    }

    return position;
}
