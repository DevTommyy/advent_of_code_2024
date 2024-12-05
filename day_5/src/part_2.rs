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

fn validate_and_fix_order(update: &[i32], rules: &[Vec<i32>]) -> Vec<i32> {
    // don't modify the input, create a clone
    let mut fixed_update = update.to_vec();
    let mut changed = true;

    while changed {
        changed = false;
        for rule in rules {
            let (lhs, rhs) = (rule[0], rule[1]);

            // the rule is contained in the update
            if let (Some(pos_lhs), Some(pos_rhs)) = (
                fixed_update.iter().position(|&p| p == lhs),
                fixed_update.iter().position(|&p| p == rhs),
            ) {
                // check rule validity
                if pos_rhs < pos_lhs {
                    // if the rule is inverted swap them
                    fixed_update.swap(pos_lhs, pos_rhs);
                    changed = true;
                }
            }
        }
    }

    fixed_update
}

fn check_invalid_and_find_middle(update: &[i32], rules: &[Vec<i32>]) -> i32 {
    let fixed_update = validate_and_fix_order(update, rules);

    // if it has been modified
    if fixed_update != update {
        fixed_update[fixed_update.len() / 2]
    } else {
        // it was already valid
        0
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (rules, updates) = parse_input(&filename);

    let output: i32 = updates
        .iter()
        .map(|update| check_invalid_and_find_middle(update, &rules))
        .sum();

    println!("{output}");
}
