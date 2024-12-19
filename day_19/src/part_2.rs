use std::collections::HashMap;
use std::fs;

fn parse_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string(filename).expect("failed to read file");
    let mut split = input.split("\n\n");

    let patterns: Vec<_> = split
        .next()
        .unwrap()
        .split(",")
        .map(|str| str.trim().to_owned())
        .collect();
    let designs: Vec<_> = split
        .next()
        .unwrap()
        .lines()
        .map(|str| str.trim().to_owned())
        .collect();

    (patterns, designs)
}

fn count_arrangements<'a>(
    remaining: &'a str,
    patterns: &Vec<String>,
    already_checked: &mut HashMap<&'a str, usize>,
) -> usize {
    if remaining.is_empty() {
        return 1;
    }

    if let Some(&cached_result) = already_checked.get(remaining) {
        return cached_result;
    }

    let mut total_ways = 0;

    for pattern in patterns {
        if remaining.starts_with(pattern) {
            let next_remaining = &remaining[pattern.len()..];
            total_ways += count_arrangements(next_remaining, patterns, already_checked);
        }
    }

    already_checked.insert(remaining, total_ways);
    total_ways
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (patterns, designs) = parse_input(&filename);

    let mut output = 0;

    for design in designs {
        let mut already_checked = HashMap::new();
        let ways = count_arrangements(&design, &patterns, &mut already_checked);
        output += ways;
    }

    println!("{output}");
}
