use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn parse_input(filename: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut free_spaces = HashSet::new();
    let mut start = (0, 0);

    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char != '#' {
                    free_spaces.insert((x as i32, y as i32));
                }
                if char == 'S' {
                    start = (x as i32, y as i32);
                }
            });
        });

    (start, free_spaces)
}

// adaptation of dijkstra of some days ago
fn dijkstra(start: (i32, i32), free_spaces: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), i32> {
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert(start, 0);

    to_visit.push((0, start));

    while let Some((score, (cx, cy))) = to_visit.pop() {
        let score = -score;

        if visited.get(&(cx, cy)).map_or(false, |&v| v < score) {
            continue;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (cx + dx, cy + dy);
            if free_spaces.contains(&np) && visited.get(&np).map_or(true, |&v| v > score + 1) {
                visited.insert(np, score + 1);
                to_visit.push((-(score + 1), np));
            }
        }
    }

    visited
}

fn get_savings(distances: &HashMap<(i32, i32), i32>) -> i32 {
    distances
        .keys()
        .flat_map(|&(x, y)| {
            (-20..=20).flat_map(move |dx| {
                (-20..=20).flat_map(move |dy| {
                    let next_pos = (x + dx, y + dy);
                    let cheat_cost = dx.abs() + dy.abs();
                    if cheat_cost > 20 {
                        return None;
                    }
                    if let Some(&initial_cost) = distances.get(&(x, y)) {
                        if let Some(&np_cost) = distances.get(&next_pos) {
                            let diff = initial_cost - np_cost - cheat_cost;
                            if diff >= 100 {
                                return Some(1);
                            }
                        }
                    }
                    None
                })
            })
        })
        .sum()
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (start, free_spaces) = parse_input(&filename);
    let distances = dijkstra(start, &free_spaces);

    let output = get_savings(&distances);

    println!("{}", output);
}
