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

fn can_make_design(remaining: &str, patterns: &Vec<String>) -> bool {
    if remaining.is_empty() {
        return true;
    }

    for pattern in patterns {
        if remaining.starts_with(pattern) && can_make_design(&remaining[pattern.len()..], patterns)
        {
            return true;
        }
    }

    false
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (patterns, designs) = parse_input(&filename);

    let mut output = 0;

    for design in designs {
        let is_possible = can_make_design(&design, &patterns);
        output += is_possible as usize;
    }

    println!("{output}");
}
