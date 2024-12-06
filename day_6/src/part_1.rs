use std::fs;

// i wanted to make this solution a bit more rust style
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

fn count_path(map: &[Vec<char>]) -> i32 {
    map.iter()
        .map(|line| line.iter().filter(|&&c| c == 'X').count() as i32)
        .sum::<i32>()
        + 1
}

impl Guard {
    fn turn_right_and_advance(&mut self, map: &[Vec<char>]) {
        let new_direction = self.direction.next();
        let (dx, dy) = new_direction.value();

        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if position_out_of_bounds(new_x, new_y, map) {
            self.exited = true;
        } else {
            self.x = new_x;
            self.y = new_y;
            self.direction = new_direction;
        }
    }

    fn advance(&mut self, map: &[Vec<char>]) {
        let (dx, dy) = self.direction.value();

        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if position_out_of_bounds(new_x, new_y, map) {
            self.exited = true;
        } else if is_block(new_x, new_y, map) {
            self.turn_right_and_advance(map);
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn play(&mut self, map: &mut [Vec<char>]) {
        while !self.exited {
            self.advance(map);
            map[self.y as usize][self.x as usize] = 'X';
        }
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

fn main() {
    let filename = advent_of_code_2024::load_input();

    let mut map = parse_input(&filename);
    let mut guard = find_guard(&map);

    guard.play(&mut map);
    let output = count_path(&map);

    println!("{output}");
}
