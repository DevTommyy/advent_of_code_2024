use std::collections::HashSet;
use std::fs;

static DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // top
    (-1, 0), // left
    (1, 0),  // right
    (0, 1),  // bottom
];

fn is_out_of_bounds(x: isize, y: isize, max_x: usize, max_y: usize) -> bool {
    x < 0 || x >= max_x as isize || y < 0 || y >= max_y as isize
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn find_group(
    key: char,
    start: (usize, usize),
    map: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut stack = vec![(start.0, start.1)];
    let mut area = 0;
    let mut perimeter = 0;

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        area += 1;

        for (dx, dy) in DIRECTIONS {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if !is_out_of_bounds(new_x, new_y, map[0].len(), map.len()) {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if map[new_y][new_x] != key {
                    perimeter += 1;
                } else if !visited.contains(&(new_x, new_y)) {
                    stack.push((new_x, new_y));
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

fn calculate_total(map: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited.contains(&(x, y)) {
                let key = map[y][x];
                let (area, perimeter) = find_group(key, (x, y), map, &mut visited);

                let price = area * perimeter;
                total_price += price;
            }
        }
    }

    total_price
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let output = calculate_total(&input);
    println!("{output}");
}
