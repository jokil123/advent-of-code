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
        let a: Vec<String> = line.split(" ").collect();

        let instruction = line.split(" ").collect();
    }

    return instructions;
}

fn run_instructions(instructions: Vec<Instruction>) -> vec![i64, i64] {
    let mut position: Vec<i64> = Vec::new();
    for instruction in instructions {
        match instruction.instruction_name {
            UP => {
                position[1] += instruction.amount;
            }
            DOWN => {
                position[1] -= instruction.amount;
            }
            FORWARD => {
                position[0] += instruction.amount;
            }
        }
    }

    return position;
}
