use std::{collections::HashMap, fs};

fn parse_input(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn next_secret(mut num: i64) -> i64 {
    num ^= (num * 64) % 16777216;
    num %= 16777216;

    num ^= (num / 32) % 16777216;
    num %= 16777216;

    num ^= (num * 2048) % 16777216;
    num %= 16777216;

    num
}

fn seq(secret: i64, length: usize) -> (Vec<i64>, Vec<i64>) {
    let mut prices = vec![];
    let mut changes = vec![];
    let mut current = secret;
    for _ in 1..=length {
        let previous = current;
        current = next_secret(current);
        prices.push(current % 10);
        changes.push((current % 10) - (previous % 10));
    }
    (prices, changes)
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let secrets: Vec<i64> = parse_input(&filename);

    let mut sales = HashMap::new();

    // iterator over sequences
    let b = secrets.into_iter().map(|secret| {
        let (prices, changes) = seq(secret, 2000);
        let mut bananas = HashMap::new();
        let n = prices.len();
        for i in 3..n {
            if let [a, b, c, d] = changes[i - 3..=i] {
                let sequence = (a, b, c, d);
                let price = prices[i];
                bananas.entry(sequence).or_insert(price);
            }
        }
        bananas
    });

    for bananas in b {
        for (k, v) in bananas {
            *sales.entry(k).or_insert(0) += v;
        }
    }

    let output = sales.into_values().max().unwrap();
    println!("{output}");
}
