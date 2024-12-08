use std::{
    collections::{HashMap, HashSet},
    fs,
};

// x, y
type Position = (i32, i32);

fn parse_input(filename: &str) -> (HashMap<char, Vec<Position>>, usize, usize) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let mut map: HashMap<char, Vec<Position>> = HashMap::new();
    let mut row_length = 0;
    let mut col_length = 0;

    for (y, line) in content.lines().enumerate() {
        row_length = y + 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_default().push((x as i32, y as i32));
            }
            col_length = col_length.max(x + 1);
        }
    }

    (map, row_length, col_length)
}

fn is_out_of_bounds(x: i32, y: i32, max_x: usize, max_y: usize) -> bool {
    x < 0 || x >= max_x as i32 || y < 0 || y >= max_y as i32
}

fn find_antinodes(positions: &[Position], set: &mut HashSet<Position>) {
    for first in positions {
        for other in positions {
            if other != first {
                let dx = other.0 - first.0;
                let dy = other.1 - first.1;

                let first_antinode: Position = (other.0 + dx, other.1 + dy);
                let second_antinode: Position = (first.0 - dx, first.1 - dy);

                set.insert(first_antinode);
                set.insert(second_antinode);
            }
        }
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (input, max_x, max_y) = parse_input(&filename);
    let mut antinodes = HashSet::new();

    input
        .values()
        .for_each(|val| find_antinodes(val, &mut antinodes));

    let output = antinodes
        .iter()
        .filter(|(x, y)| !is_out_of_bounds(*x, *y, max_x, max_y))
        .count();

    println!("{output}");
}
