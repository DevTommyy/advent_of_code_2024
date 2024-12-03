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

fn count_instances(num: i32, list2: &[i32]) -> i32 {
    let mut count = 0;
    let first_idx = list2.iter().position(|&x| x == num);

    if first_idx.is_none() {
        0
    } else {
        let mut idx = first_idx.unwrap();
        while list2[idx] == num {
            count += 1;
            idx += 1;
        }

        count
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();

    let (list1, list2) = process_input(&filename);

    let total: i32 = list1
        .into_iter()
        .map(|num| {
            let instances = count_instances(num, &list2);
            num * instances
        })
        .sum();

    println!("{total}");
}
