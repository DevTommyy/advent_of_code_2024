use std::fs;

fn process_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(filename).expect("failed to read file");

    // processed input
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    input
        .lines()
        .flat_map(|line| line.split("   "))
        .enumerate()
        .for_each(|(idx, item)| {
            let num: i32 = item.parse().expect("failed to parse number");
            if idx % 2 == 0 {
                list1.push(num);
            } else {
                list2.push(num);
            }
        });

    // IMPORTANT sort them
    list1.sort();
    list2.sort();

    (list1, list2)
}

fn calculate_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs()
    }

    total
}

fn main() {
    let filename = advent_of_code_2024::load_input();

    let (list1, list2) = process_input(&filename);

    let total = calculate_distance(list1, list2);
    println!("{total}");
}