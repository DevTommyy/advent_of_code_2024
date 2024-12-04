use std::{char, fs};

// if you notice, checking the reciprocals of the diagonals and then reversing the signs of these 2
// is enough
static CORNERS: [(isize, isize); 2] = [
    (-1, -1), // upper left corner
    (-1, 1),  // bottom left corner
              // (1, -1),  // upper right corner
              // (1, 1),   // bottom right corner
];

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

// awful brute force approach, could be refactored much better but the concept is there
fn find_match(input: &[Vec<char>], x: usize, y: usize, valid_count: &mut u32) {
    // calculate diagonal coordinates
    let upper_dx = CORNERS[0].0;
    let upper_dy = CORNERS[0].1;
    let upper_left_x = x as isize + upper_dx;
    let upper_left_y = y as isize + upper_dy;

    let bottom_dx = CORNERS[1].0;
    let bottom_dy = CORNERS[1].1;
    let bottom_left_x = x as isize + bottom_dx;
    let bottom_left_y = y as isize + bottom_dy;

    let upper_reciprocal_x = x as isize - upper_dx;
    let upper_reciprocal_y = y as isize - upper_dy;

    let bottom_reciprocal_x = x as isize - bottom_dx;
    let bottom_reciprocal_y = y as isize - bottom_dy;

    // check if all the indices are within bounds
    if upper_left_x >= 0
        && upper_left_y >= 0
        && bottom_left_x >= 0
        && bottom_left_y >= 0
        && upper_reciprocal_x >= 0
        && upper_reciprocal_y >= 0
        && bottom_reciprocal_x >= 0
        && bottom_reciprocal_y >= 0
        && (upper_left_x as usize) < input.len()
        && (upper_left_y as usize) < input[0].len()
        && (bottom_left_x as usize) < input.len()
        && (bottom_left_y as usize) < input[0].len()
        && (upper_reciprocal_x as usize) < input.len()
        && (upper_reciprocal_y as usize) < input[0].len()
        && (bottom_reciprocal_x as usize) < input.len()
        && (bottom_reciprocal_y as usize) < input[0].len()
    {
        // convert to usize after checking bounds
        let upper_left_x_usize = upper_left_x as usize;
        let upper_left_y_usize = upper_left_y as usize;
        let bottom_left_x_usize = bottom_left_x as usize;
        let bottom_left_y_usize = bottom_left_y as usize;
        let upper_reciprocal_x_usize = upper_reciprocal_x as usize;
        let upper_reciprocal_y_usize = upper_reciprocal_y as usize;
        let bottom_reciprocal_x_usize = bottom_reciprocal_x as usize;
        let bottom_reciprocal_y_usize = bottom_reciprocal_y as usize;

        if ((input[upper_left_x_usize][upper_left_y_usize] == 'M'
            || input[upper_left_x_usize][upper_left_y_usize] == 'S')
            && (input[upper_reciprocal_x_usize][upper_reciprocal_y_usize] == 'M'
                || input[upper_reciprocal_x_usize][upper_reciprocal_y_usize] == 'S'))
            && (input[upper_left_x_usize][upper_left_y_usize]
                != input[upper_reciprocal_x_usize][upper_reciprocal_y_usize])
            && ((input[bottom_left_x_usize][bottom_left_y_usize] == 'M'
                || input[bottom_left_x_usize][bottom_left_y_usize] == 'S')
                && (input[bottom_reciprocal_x_usize][bottom_reciprocal_y_usize] == 'M'
                    || input[bottom_reciprocal_x_usize][bottom_reciprocal_y_usize] == 'S'))
            && (input[bottom_left_x_usize][bottom_left_y_usize]
                != input[bottom_reciprocal_x_usize][bottom_reciprocal_y_usize])
        {
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
            // base is now A
            if input[x][y] == 'A' {
                find_match(&input, x, y, &mut count);
            }
        }
    }

    println!("{count}");
}
