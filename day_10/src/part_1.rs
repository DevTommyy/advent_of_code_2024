use std::{collections::HashSet, fs};

static DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // top
    (-1, 0), // left
    (1, 0),  // right
    (0, 1),  // bottom
];

fn is_out_of_bounds(x: isize, y: isize, max_x: usize, max_y: usize) -> bool {
    x < 0 || x >= max_x as isize || y < 0 || y >= max_y as isize
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn check_trailhead(start: (usize, usize), map: &[Vec<u32>]) -> usize {
    let mut stack = vec![(0u32, start.0, start.1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((current_value, x, y)) = stack.pop() {
        if current_value == 9 {
            visited.insert((x, y));
        } else {
            let next_value = current_value + 1;
            for (dx, dy) in DIRECTIONS {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if !is_out_of_bounds(new_x, new_y, map[0].len(), map.len())
                    && map[new_y as usize][new_x as usize] == next_value
                {
                    stack.push((next_value, new_x as usize, new_y as usize));
                }
            }
        }
    }
    visited.len()
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let zeros: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let output: usize = zeros
        .into_iter()
        .map(|pos| check_trailhead(pos, &input))
        .sum();

    println!("{output}");
}
