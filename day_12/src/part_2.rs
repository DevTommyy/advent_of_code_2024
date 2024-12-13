use std::{
    collections::{HashMap, HashSet},
    fs,
};

static DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

static CORNERS: [[(isize, isize); 3]; 4] = [
    [(-1, -1), (-1, 0), (0, -1)],
    [(1, -1), (1, 0), (0, -1)],
    [(1, 1), (1, 0), (0, 1)],
    [(-1, 1), (-1, 0), (0, 1)],
];

fn parse_input(filename: &str) -> HashMap<(isize, isize), char> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as isize, y as isize), char))
        })
        .collect()
}

fn neighbors(
    groups: &HashMap<(isize, isize), char>,
    group: &(isize, isize),
) -> Vec<(isize, isize)> {
    DIRECTIONS
        .iter()
        .map(|&offset| (group.0 + offset.0, group.1 + offset.1))
        .filter(|neighbor| groups.contains_key(neighbor))
        .collect()
}

fn corners(groups: &HashMap<(isize, isize), char>, group: &(isize, isize)) -> usize {
    CORNERS
        .iter()
        .filter(|corner| {
            let opposite = groups.get(&(group.0 + corner[0].0, group.1 + corner[0].1));
            let first = groups.get(&(group.0 + corner[1].0, group.1 + corner[1].1));
            let second = groups.get(&(group.0 + corner[2].0, group.1 + corner[2].1));
            let current = groups.get(group);

            (current != second && current != first)
                || (current == second && current == first && current != opposite)
        })
        .count()
}

fn find_groups(
    input: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    item: char,
    group: &mut HashSet<(isize, isize)>,
) {
    let mut stack = vec![*start];

    while let Some(current) = stack.pop() {
        if !group.insert(current) {
            continue;
        }

        for neighbor in neighbors(input, &current) {
            if input[&neighbor] == item && !group.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let mut regions = Vec::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for position in input.keys() {
        if visited.contains(position) {
            continue;
        }

        let mut region = HashSet::new();
        find_groups(&input, position, input[position], &mut region);

        visited.extend(&region);
        regions.push(region);
    }

    let output = regions
        .iter()
        .map(|region| {
            region
                .iter()
                .map(|group| corners(&input, group))
                .sum::<usize>()
                * region.len()
        })
        .sum::<usize>();

    println!("{output}");
}
