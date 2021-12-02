use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let mut last_depth: u64 = u64::MAX;

    let mut depth_increase_count: i64 = 0;

    for line in lines {
        println!("{}",line);
        let current_depth = match line.parse() {
            Ok(num) => num,
            Err(_) => {continue;}

        };

        if current_depth > last_depth {
            depth_increase_count += 1;
        }

        last_depth = current_depth;
    }

    println!("Depth Increased {} times", depth_increase_count);
}