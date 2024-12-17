use std::fs;

fn parse_input(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .split("\n\n")
        .last()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(",")
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

// for my input it is doing this:
//
// b = a % 8
// b = b ^ 1
// c = a >> b
// b = b ^ c
// a = a >> 3
// b = b ^ 4
// out(b % 8)
// if a != 0: jump 0

// note that this works only for my input since its a reverse engineer of it
fn execute(instructions: &[i64], output: i64) -> Option<i64> {
    for try_val in 0..8 {
        let a = (output << 3) | try_val;
        let mut b = a % 8;
        b ^= 1;
        let c = a >> b;
        b ^= c;
        b ^= 4;

        // i need this base case since i remove the last instruction in the recursion
        if instructions.len() == 1 {
            if b % 8 == instructions[0] {
                return Some(a);
            } else {
                // try next val
                continue;
            }
        }
        if b % 8 == instructions[instructions.len() - 1] {
            if let Some(prev_a) = execute(&instructions[0..instructions.len() - 1], a) {
                return Some(prev_a);
            }
        }
    }
    None
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let instructions = parse_input(&filename);

    let output = execute(&instructions, 0);

    println!("{output:?}");
}
