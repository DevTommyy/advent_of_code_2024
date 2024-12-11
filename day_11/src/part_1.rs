use std::fs;

fn parse_input(filename: &str) -> Vec<i128> {
    fs::read_to_string(filename)
        .expect("failed to open file")
        .split_whitespace()
        .map(|num| num.parse::<i128>().unwrap())
        .collect()
}

fn blink(input: &[i128], total: i32) -> Vec<i128> {
    let mut new = Vec::new();
    if total >= 25 {
        return input.to_vec();
    }
    for &num in input {
        match num {
            0 => new.push(1),
            x if x.to_string().len() % 2 == 0 => {
                let x_str = x.to_string();
                let left = x_str[0..x_str.len() / 2].parse::<i128>().unwrap();
                let right = x_str[x_str.len() / 2..].parse::<i128>().unwrap();

                new.push(left);
                new.push(right);
            }
            _ => new.push(num * 2024),
        }
    }
    blink(&new, total + 1)
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let result = blink(&input, 0);
    println!("{}", result.len());
}
