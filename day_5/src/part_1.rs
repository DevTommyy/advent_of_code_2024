use std::fs;

fn parse_input(filename: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = fs::read_to_string(filename).expect("failed to read file");

    let mut split = input.split("\n\n");

    let unparsed_rules = split.next().unwrap();
    let unparsed_updates = split.next().unwrap();

    let rules: Vec<Vec<i32>> = unparsed_rules
        .lines()
        .map(|line| {
            line.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let updates: Vec<Vec<i32>> = unparsed_updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn find_rule_rhs(rules: &[Vec<i32>], lhs: i32) -> Vec<i32> {
    rules
        .iter()
        .filter(|rule| rule[0] == lhs)
        .map(|rule| rule[1])
        .collect()
}

fn is_valid_update(update: &[i32], rules: &[Vec<i32>]) -> bool {
    let mut valid = true;
    for idx in 0..update.len() {
        if idx == 0 && !rules.iter().any(|rule| rule[0] == update[idx]) {
            return false;
        }

        let rules_rhs = find_rule_rhs(rules, update[idx]);
        if !rules_rhs.is_empty() {
            for rhs in rules_rhs {
                let mut j = 0;

                while j < idx {
                    if update[j] == rhs {
                        valid = false;
                        break;
                    }
                    j += 1;
                }
            }
        }
    }

    valid
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (rules, updates) = parse_input(&filename);

    let output: i32 = updates
        .iter()
        .filter(|update| is_valid_update(update, &rules))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    println!("{output}");
}
