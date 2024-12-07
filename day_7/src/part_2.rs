use std::{collections::HashMap, fs};

fn parse_input(filename: &str) -> HashMap<i64, Vec<i32>> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let key = parts.next()?.trim().parse::<i64>().ok()?;

            let values = parts
                .next()?
                .split_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            Some((key, values))
        })
        .collect()
}

fn check_total(total: i64, values: &[i32]) -> bool {
    let mut stack = vec![(0i64, 0usize)]; // current value, current index

    while let Some((current_value, index)) = stack.pop() {
        // if you have fully tried that combination check if what you got
        // is what you needed
        if index == values.len() {
            if current_value == total {
                return true;
            }
        } else {
            // from each number try adding, multiplying and combining and so on
            let next_value = values[index] as i64;
            stack.push((current_value + next_value, index + 1));
            stack.push((current_value * next_value, index + 1));
            stack.push((
                current_value * 10i64.pow(next_value.to_string().len() as u32) + next_value,
                index + 1,
            ));
        }
    }

    false
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let map = parse_input(&filename);

    let output = map
        .iter()
        .map(|(&key, val)| if check_total(key, val) { key } else { 0 })
        .sum::<i64>();

    println!("{output}");
}
