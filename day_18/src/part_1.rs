use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn parse_input(filename: &str) -> Vec<(usize, usize)> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

fn is_out_of_bounds(x: isize, y: isize, maze: &[Vec<char>]) -> bool {
    x < 0 || x >= maze[0].len() as isize || y < 0 || y >= maze.len() as isize
}

fn simulate_falling_bytes(bytes: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut maze = vec![vec!['.'; 71]; 71];
    for &(x, y) in bytes.iter().take(1024) {
        maze[y][x] = '#';
    }
    maze
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    g_score: usize,
    heuristic: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.g_score + self.heuristic)
            .cmp(&(other.g_score + other.heuristic))
            .reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn neighbors(maze: &[Vec<char>], position: (usize, usize)) -> Vec<(usize, usize)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    directions
        .iter()
        .filter_map(|&(dx, dy)| {
            let nx = position.0 as isize + dx;
            let ny = position.1 as isize + dy;

            if !is_out_of_bounds(nx, ny, maze) && maze[ny as usize][nx as usize] != '#' {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
}

fn a_star(
    maze: &[Vec<char>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut closed_set = HashMap::new();

    g_score.insert(start, 0);

    open_set.push(Node {
        position: start,
        g_score: 0,
        heuristic: heuristic(start, goal),
    });

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            let mut path = vec![current.position];
            let mut position = current.position;

            while let Some(&prev) = came_from.get(&position) {
                path.push(prev);
                position = prev;
            }

            path.reverse();
            return Some((path, g_score[&goal]));
        }

        if closed_set.contains_key(&current.position) {
            continue;
        }
        closed_set.insert(current.position, true);

        for neighbor in neighbors(maze, current.position) {
            let tentative_g_score = g_score.get(&current.position).unwrap_or(&usize::MAX) + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    position: neighbor,
                    g_score: tentative_g_score,
                    heuristic: heuristic(neighbor, goal),
                });
            }
        }
    }

    None
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let bytes = parse_input(&filename);
    let maze = simulate_falling_bytes(bytes);
    let start = (0, 0);
    let goal = (70, 70);

    let (_path, cost) = a_star(&maze, start, goal).unwrap();
    println!("{cost}");
}
