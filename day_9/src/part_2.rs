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
    let filtered_input: Vec<i32> = input.iter().filter(|&&i| i != -1).copied().collect();
    let mut output = input.clone();

    let mut num = *filtered_input.iter().next_back().unwrap();
    while num != 0 {
        let mut chunk_len = filtered_input.iter().rev().filter(|&&i| i == num).count();
        let start_chunk_idx = input.iter().position(|&x| x == num).unwrap();

        let mut free_chunk_len = 0;
        let mut start_idx = 0usize;

        for idx in 0..output.len() - 1 {
            match (output[idx], output[idx + 1]) {
                (x, -1) if x != -1 => start_idx = idx + 1,
                (-1, -1) => free_chunk_len += 1,
                (-1, x) if x != -1 => {
                    free_chunk_len += 1;
                    if free_chunk_len >= chunk_len && start_idx < start_chunk_idx {
                        while chunk_len > 0 {
                            output.swap(start_idx + chunk_len - 1, start_chunk_idx + chunk_len - 1);
                            chunk_len -= 1
                        }
                        break;
                    } else {
                        free_chunk_len = 0;
                    }
                }
                (x, y) if x != -1 && y != -1 => continue,
                _ => unreachable!(),
            }
        }
        num -= 1;
    }
    output
}

fn calculate(compacte_input: Vec<i32>) -> i128 {
    compacte_input
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != -1)
        .fold(0i128, |acc, (idx, val)| acc + (*val as i128 * idx as i128))
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let compacted_input = compact_filesystem(input);

    let output = calculate(compacted_input);
    println!("{output}")
}
