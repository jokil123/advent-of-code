use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let mut depths: Vec<i64> = Vec::new();

    for line in lines {

        match line.trim().parse() {
            Ok(num) => {depths.push(num)},
            Err(_) => {println!("failed to read number");continue;}

        };
    }

    println!("Depth Increased {} times", count_increases(create_groups(&depths)));
}

fn create_groups(depths: &Vec<i64>) -> Vec<i64> {
    let mut groups: Vec<i64> = Vec::new();

    for i in 1..(depths.len()-1) {
        let before = depths[i-1];
        let current = depths[i];
        let after = depths[i+1];

        groups.push(before+current+after);
    }

    return groups;
}

fn count_increases(windows: Vec<i64>) -> i64 {
    let mut last_depth: i64 = i64::MAX;

    let mut depth_increase_count: i64 = 0;

    for depth in windows {

        if depth > last_depth {
            depth_increase_count += 1;
        }

        last_depth = depth;
    }

    return depth_increase_count
}