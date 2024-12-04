use std::{char, fs};

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // upper left corner
    (0, -1),  // top
    (1, -1),  // upper right corner
    (-1, 0),  // left
    (1, 0),   // right
    (-1, 1),  // bottom left corner
    (0, 1),   // bottom
    (1, 1),   // bottom right corner
];

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn find_match(input: &[Vec<char>], x: usize, y: usize, valid_count: &mut u32) {
    // since i start from the base 'X' i add the delta and check only for the remaning pattern
    let sequence = ['M', 'A', 'S'];

    for (dx, dy) in DIRECTIONS {
        let mut new_x = x as isize + dx;
        let mut new_y = y as isize + dy;
        let mut found = true;

        for &letter in &sequence {
            // if new indexes are out of bounds or input of those indexes isnt a match
            if new_x < 0
                || new_y < 0
                || new_x as usize >= input.len()
                || new_y as usize >= input[0].len()
                || input[new_x as usize][new_y as usize] != letter
            {
                found = false;
                break;
            }
            // advance in that direction
            new_x += dx;
            new_y += dy;
        }

        if found {
            *valid_count += 1;
        }
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = read_input(&filename);
    let mut count: u32 = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            // find the base for each match
            if input[x][y] == 'X' {
                find_match(&input, x, y, &mut count);
            }
        }
    }

    println!("{count}");
}
