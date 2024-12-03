use std::fs;

// i hate regexes
fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = fs::read_to_string(&filename).expect("failed to read file");

    let output: i32 = input
        .split("mul")
        .skip(1)
        .map(|s| {
            if let Some(end_idx) = s.find(")") {
                let s = &s[1..end_idx];

                let nums: Vec<i32> = s
                    .split(",")
                    .flat_map(|maybe_num| maybe_num.parse::<i32>())
                    .collect();

                if nums.len() == 2 {
                    nums[0] * nums[1]
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    println!("{output}");
}
