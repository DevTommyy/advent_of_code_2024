use std::fs;

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let lines = fs::read_to_string(filename).expect("failed to read file");
    lines
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().expect("failed to parse"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe_second_part(report: Vec<i32>) -> bool {
    if is_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn is_safe(report: &[i32]) -> bool {
    report
        .iter()
        .is_sorted_by(|&&a, &&b| a > b && (a - b).abs() <= 3 && (a - b).abs() >= 1)
        || report
            .iter()
            .is_sorted_by(|&&a, &&b| a < b && (a - b).abs() <= 3 && (a - b).abs() >= 1)
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = read_input(&filename);

    let output = input
        .into_iter()
        .map(|report| is_safe_second_part(report) as i32)
        .sum::<i32>();

    println!("{output}");
}
