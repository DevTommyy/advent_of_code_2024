use std::fs;

fn parse_input(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn calculate(mut num: i64) -> i64 {
    for _ in 0..2000 {
        num ^= (num * 64) % 16777216;
        num %= 16777216;
        num ^= (num / 32) % 16777216;
        num %= 16777216;
        num ^= (num * 2048) % 16777216;
        num %= 16777216;
    }
    num
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let result: i64 = input.iter().map(|&num| calculate(num)).sum();
    println!("{result}");
}
