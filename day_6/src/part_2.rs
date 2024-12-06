use std::collections::HashSet;
use std::fs;

#[derive(Clone)]
enum Translation {
    TopToRight,
    RightToBottom,
    BottomToLeft,
    LeftToTop,
}

impl Translation {
    fn value(&self) -> (i32, i32) {
        match self {
            Translation::TopToRight => (1, 0),
            Translation::RightToBottom => (0, 1),
            Translation::BottomToLeft => (-1, 0),
            Translation::LeftToTop => (0, -1),
        }
    }

    fn next(&self) -> Self {
        match self {
            Translation::TopToRight => Translation::RightToBottom,
            Translation::RightToBottom => Translation::BottomToLeft,
            Translation::BottomToLeft => Translation::LeftToTop,
            Translation::LeftToTop => Self::TopToRight,
        }
    }
}

struct Guard {
    x: i32,
    y: i32,
    direction: Translation,
    exited: bool,
}

fn position_out_of_bounds(x: i32, y: i32, map: &[Vec<char>]) -> bool {
    x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32
}

fn is_block(x: i32, y: i32, map: &[Vec<char>]) -> bool {
    map[y as usize][x as usize] == '#'
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = self.direction.next();
    }

    fn advance(&mut self, map: &[Vec<char>]) {
        let (dx, dy) = self.direction.value();

        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if position_out_of_bounds(new_x, new_y, map) {
            self.exited = true;
        } else if is_block(new_x, new_y, map) {
            self.turn_right();
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    // WHAT'S THE INTUITION?
    // a loop is when the guard reaches the same position twice (obviously) but the important part
    // is that its direction it's the same, if it is the case, then there is a loop with the added
    // block, so instead of adding a block and checking the path that it does, you can just add a
    // block and check if the (position, direction) is the same 2 times
    fn play_until_loop(&mut self, map: &[Vec<char>]) -> bool {
        // set with coordinate and direction of the guard
        let mut visited = HashSet::new();

        while !self.exited {
            // save the state
            let state = (self.x, self.y, self.direction.value());
            // check if it was already visited with the same direction
            if !visited.insert(state) {
                // WE HAVE A LOOP
                return true;
            }
            self.advance(map);
        }
        false
    }
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn find_guard(map: &[Vec<char>]) -> Guard {
    let (y, x) = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &ch)| ch == '^')
                .map(|(x, _)| (y, x))
        })
        .expect("initial position not found");

    Guard {
        x: x as i32,
        y: y as i32,
        direction: Translation::LeftToTop,
        exited: false,
    }
}

fn count_valid_obstruction_positions(map: &[Vec<char>], start: &Guard) -> usize {
    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // if there is a point where i can place an obstace place it and check for loop
            if map[y][x] == '.' {
                let mut modified_map = map.to_vec();
                modified_map[y][x] = '#';

                // clone the guard to preserve the initial state
                // for the other runs
                let mut guard = Guard {
                    x: start.x,
                    y: start.y,
                    direction: start.direction.clone(),
                    exited: false,
                };

                if guard.play_until_loop(&modified_map) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let filename = advent_of_code_2024::load_input();

    let map = parse_input(&filename);
    let guard = find_guard(&map);

    let output = count_valid_obstruction_positions(&map, &guard);

    println!("{output}");
}
