use std::fs;

fn parse_mul(s: &str) -> i32 {
    if let Some(end_idx) = s.find(")") {
        let s = &s[1..end_idx];

        let nums: Vec<i32> = s
            .split(",")
            .flat_map(|maybe_num| maybe_num.trim().parse::<i32>())
            .collect();

        if nums.len() == 2 {
            nums[0] * nums[1]
        } else {
            0
        }
    } else {
        0
    }
}

// i hate regexes
fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = fs::read_to_string(&filename).expect("failed to read file");
    let mut sections = input.split("don't");

    // before the first dont just apply the logic as part one
    let before_first_dont = sections.next().unwrap_or_default();
    let first_result: i32 = before_first_dont.split("mul").map(parse_mul).sum();

    let subsequent_result: i32 = sections
        .flat_map(|s| s.split("do").skip(1))
        .flat_map(|s| s.split("mul"))
        .map(parse_mul)
        .sum();

    let output = first_result + subsequent_result;
    println!("{output}");
}
