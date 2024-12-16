use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn parse_input(filename: &str) -> (HashSet<(i32, i32)>, (i32, i32), (i32, i32)) {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut free_spaces = HashSet::new();

    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    'E' => {
                        goal = (x as i32, y as i32);
                        free_spaces.insert((x as i32, y as i32));
                    }
                    'S' => start = (x as i32, y as i32),
                    '.' => {
                        free_spaces.insert((x as i32, y as i32));
                    }
                    _ => {}
                };
            })
        });

    (free_spaces, start, goal)
}

fn dijkstra(
    start: (i32, i32),
    free_spaces: &HashSet<(i32, i32)>,
) -> HashMap<((i32, i32), char), i32> {
    let deltas: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    let rot = ['>', 'v', '<', '^'];

    let mut to_visit = BinaryHeap::new();
    let mut visited: HashMap<((i32, i32), char), i32> = HashMap::new();
    visited.insert((start, '>'), 0);

    to_visit.push((0, '>', start));

    while let Some((score, cd, (cx, cy))) = to_visit.pop() {
        let score = -score;

        if visited.get(&((cx, cy), cd)).map_or(false, |&v| v < score) {
            continue;
        }

        let (dx, dy) = deltas[&cd];

        // try forward
        let np = (cx + dx, cy + dy);
        if free_spaces.contains(&np) && visited.get(&(np, cd)).map_or(true, |&v| v > score + 1) {
            visited.insert((np, cd), score + 1);
            to_visit.push((-(score + 1), cd, np));
        }

        // try rotating
        for dr in [-1, 1] {
            let nd = rot[(((rot.iter().position(|&r| r == cd).unwrap() as i32) + dr).rem_euclid(4))
                as usize];
            if visited
                .get(&((cx, cy), nd))
                .map_or(true, |&v| v > score + 1000)
            {
                visited.insert(((cx, cy), nd), score + 1000);
                to_visit.push((-(score + 1000), nd, (cx, cy)));
            }
        }
    }

    visited
}

fn trace_back(
    visited: &HashMap<((i32, i32), char), i32>,
    target_state: ((i32, i32), char),
) -> HashSet<(i32, i32)> {
    let directions: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0)), ('^', (0, -1))]);
    let rotation = ['>', 'v', '<', '^'];

    let mut to_visit = vec![target_state];
    let mut seen = HashSet::new();

    while let Some((cp, cd)) = to_visit.pop() {
        seen.insert(cp);

        let (dx, dy) = directions[&cd];
        let np = (cp.0 - dx, cp.1 - dy);

        // trace back forward
        if visited
            .get(&(np, cd))
            .map_or(false, |&v| v + 1 == visited[&(cp, cd)])
        {
            to_visit.push((np, cd));
        }

        // track back rotating
        let nd1 = rotation
            [((rotation.iter().position(|&r| r == cd).unwrap() as i32 + 1).rem_euclid(4)) as usize];
        let nd2 = rotation
            [((rotation.iter().position(|&r| r == cd).unwrap() as i32 - 1).rem_euclid(4)) as usize];

        if visited
            .get(&(cp, nd1))
            .map_or(false, |&v| v + 1000 == visited[&(cp, cd)])
        {
            to_visit.push((cp, nd1));
        }
        if visited
            .get(&(cp, nd2))
            .map_or(false, |&v| v + 1000 == visited[&(cp, cd)])
        {
            to_visit.push((cp, nd2));
        }
    }

    seen
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (free_spaces, start, goal) = parse_input(&filename);

    let visited = dijkstra(start, &free_spaces);

    // the minimum of all the scores found
    let target_score = visited
        .iter()
        .filter(|&(&(pos, _), _)| pos == goal)
        .map(|(_, &score)| score)
        .min()
        .unwrap();

    let target_state = visited
        .iter()
        .find(|&(&(pos, _), &score)| pos == goal && score == target_score)
        .map(|(&(pos, dir), _)| (pos, dir))
        .unwrap();

    let output = trace_back(&visited, target_state).len();
    println!("{output}");
}
