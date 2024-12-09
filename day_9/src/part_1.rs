use std::fs;

fn parse_input(filename: &str) -> Vec<i32> {
    let mut input = Vec::new();
    let mut count = 0;

    fs::read_to_string(filename)
        .expect("failed to read file")
        .trim()
        .chars()
        .enumerate()
        .for_each(|(idx, c)| {
            let times = c.to_digit(10).unwrap() as usize;
            if idx & 1 == 0 {
                for _ in 0..times {
                    input.push(count);
                }
                count += 1;
            } else {
                for _ in 0..times {
                    input.push(-1);
                }
            }
        });

    input
}

fn compact_filesystem(input: Vec<i32>) -> Vec<i32> {
    let mut clone: Vec<i32> = input.clone();

    for (idx, _) in clone
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != -1)
        .rev()
    {
        if let Some(dot_pos) = clone.iter().position(|&c| c == -1) {
            if dot_pos > idx {
                break;
            }

            clone.swap(idx, dot_pos);
        }
    }

    clone
}

fn calculate(compacte_input: Vec<i32>) -> i128 {
    compacte_input
        .iter()
        .enumerate()
        .take_while(|(_, c)| **c != -1)
        .fold(0i128, |acc, (idx, val)| acc + (*val as i128 * idx as i128))
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let compacted_input = compact_filesystem(input);

    let output = calculate(compacted_input);
    println!("{output}")
}
