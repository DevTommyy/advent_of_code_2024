use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn parse_input(filename: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    let maze: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    } else if c == 'E' {
                        goal = (x, y);
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    (maze, start, goal)
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    g_score: usize,
    heuristic: usize,
    came_from: Option<(usize, usize)>,
    direction: Option<(isize, isize)>,
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

fn neighbors(
    maze: &[Vec<char>],
    position: (usize, usize),
) -> Vec<((usize, usize), (isize, isize))> {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut result = vec![];

    for (dx, dy) in directions {
        let nx = position.0 as isize + dx;
        let ny = position.1 as isize + dy;

        // no need to check out of bounds this time
        if maze[ny as usize][nx as usize] != '#' {
            result.push(((nx as usize, ny as usize), (dx, dy)));
        }
    }

    result
}

fn a_star(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<(Vec<(usize, usize)>, usize)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut direction_map: HashMap<(usize, usize), (isize, isize)> = HashMap::new();

    g_score.insert(start, 0);

    open_set.push(Node {
        position: start,
        g_score: 0,
        heuristic: heuristic(start, goal),
        came_from: None,
        direction: None,
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

            let mut turn_count = 0;
            for i in 1..path.len() - 1 {
                let (prev, curr, next) = (path[i - 1], path[i], path[i + 1]);
                let dir1 = (
                    curr.0 as isize - prev.0 as isize,
                    curr.1 as isize - prev.1 as isize,
                );
                let dir2 = (
                    next.0 as isize - curr.0 as isize,
                    next.1 as isize - curr.1 as isize,
                );
                if dir1 != dir2 {
                    turn_count += 1;
                }
            }

            let total_cost = (turn_count + 1) * 1000 + (path.len() - 1);
            return Some((path, total_cost));
        }

        for (neighbor, direction) in neighbors(&maze, current.position) {
            let turn_cost = if let Some(curr_dir) = current.direction {
                if curr_dir != direction {
                    1000
                } else {
                    0
                }
            } else {
                0
            };

            let tentative_g_score =
                g_score.get(&current.position).unwrap_or(&usize::MAX) + 1 + turn_cost;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);
                direction_map.insert(neighbor, direction);

                open_set.push(Node {
                    position: neighbor,
                    g_score: tentative_g_score,
                    heuristic: heuristic(neighbor, goal),
                    came_from: Some(current.position),
                    direction: Some(direction),
                });
            }
        }
    }

    None
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (maze, start, goal) = parse_input(&filename);
    let (_path, cost) = a_star(maze.clone(), start, goal).unwrap();

    println!("{cost}")
}
