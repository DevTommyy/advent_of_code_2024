use std::collections::HashMap;

fn parse_input(filename: &str) -> HashMap<i128, i128> {
    let input = std::fs::read_to_string(filename).expect("failed to open file");

    let mut counts = HashMap::new();
    for num in input.split_whitespace() {
        let value = num.parse::<i128>().unwrap();
        *counts.entry(value).or_insert(0) += 1;
    }
    counts
}

fn blink(mut counts: HashMap<i128, i128>, total_blinks: i32) -> i128 {
    for _ in 0..total_blinks {
        let mut new_counts = HashMap::new();

        for (&value, &count) in &counts {
            match value {
                0 => {
                    *new_counts.entry(1).or_insert(0) += count;
                }
                x if x.to_string().len() % 2 == 0 => {
                    let x_str = x.to_string();
                    let mid = x_str.len() / 2;

                    let left = x_str[0..mid].parse::<i128>().unwrap();
                    let right = x_str[mid..].parse::<i128>().unwrap();

                    *new_counts.entry(left).or_insert(0) += count;
                    *new_counts.entry(right).or_insert(0) += count;
                }
                _ => {
                    *new_counts.entry(value * 2024).or_insert(0) += count;
                }
            }
        }
        counts = new_counts;
    }

    counts.values().sum()
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let output = blink(input, 75);
    println!("{}", output);
}
