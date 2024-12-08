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

fn find_antinodes(positions: &[Position], set: &mut HashSet<Position>, max_x: usize, max_y: usize) {
    for first in positions {
        for other in positions {
            if other != first {
                set.insert(*other);
                set.insert(*first);

                let dx = other.0 - first.0;
                let dy = other.1 - first.1;

                let first_antinode: Position = (other.0 + dx, other.1 + dy);

                if !is_out_of_bounds(first_antinode.0, first_antinode.1, max_x, max_y) {
                    set.insert(first_antinode);
                    let mut antinode = first_antinode;
                    loop {
                        antinode = (antinode.0 + dx, antinode.1 + dy);
                        if is_out_of_bounds(antinode.0, antinode.1, max_x, max_y) {
                            break;
                        }
                        set.insert(antinode);
                    }
                }

                let second_antinode: Position = (first.0 - dx, first.1 - dy);

                if !is_out_of_bounds(second_antinode.0, second_antinode.1, max_x, max_y) {
                    set.insert(second_antinode);
                    let mut antinode = second_antinode;
                    loop {
                        antinode = (antinode.0 - dx, antinode.1 - dy);
                        if is_out_of_bounds(antinode.0, antinode.1, max_x, max_y) {
                            break;
                        }
                        set.insert(antinode);
                    }
                }
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
        .for_each(|val| find_antinodes(val, &mut antinodes, max_x, max_y));

    let output = antinodes.len();

    println!("{output}");
}
