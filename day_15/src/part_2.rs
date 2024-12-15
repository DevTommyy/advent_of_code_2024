use macroquad::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

enum Direction {
    Top,
    Right,
    Left,
    Bottom,
}

impl Direction {
    // x, y
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Bottom => (0, 1),
        }
    }
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
}

impl Robot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn advance(&mut self, direction: Direction, map: &mut [Vec<char>]) {
        let (dx, dy) = direction.value();
        let current_pos = (self.x, self.y);

        // find connected boxes
        if let Some(forward) = Robot::expand_forward(map, current_pos, (dx, dy)) {
            let backup = map.to_vec();

            // clear the old positions forward
            for &(x, y) in &forward {
                map[y as usize][x as usize] = '.';
            }

            // move every box
            for &(x, y) in &forward {
                let new_pos = (x + dx, y + dy);
                map[new_pos.1 as usize][new_pos.0 as usize] = backup[y as usize][x as usize];
            }

            self.x += dx;
            self.y += dy;
        }
    }

    // helper
    fn expand_forward(
        grid: &[Vec<char>],
        start: (i32, i32),
        direction: (i32, i32),
    ) -> Option<HashSet<(i32, i32)>> {
        let mut forward = vec![start];
        let mut all = HashSet::from([start]);

        let left = (-1, 0);
        let right = (1, 0);

        while let Some((x, y)) = forward.pop() {
            let next = (x + direction.0, y + direction.1);

            if all.contains(&next) {
                continue;
            }

            let cell = grid[next.1 as usize][next.0 as usize];
            match cell {
                '.' => {} // found point to move
                ']' | '[' => {
                    forward.push(next);
                    all.insert(next);

                    // if moving vertically, handle tricky horizontal connections
                    if direction.1 != 0 {
                        let other = if cell == ']' {
                            (next.0 + left.0, next.1 + left.1)
                        } else {
                            (next.0 + right.0, next.1 + right.1)
                        };
                        forward.push(other);
                        all.insert(other);
                    }
                }
                _ => return None, // blocked by a wall
            }
        }

        Some(all)
    }
}

fn parse_input(filename: &str) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
    let input = fs::read_to_string(filename).expect("failed to read file");

    let mut split = input.split("\n\n");

    let map = split.next().unwrap();
    let moves = split.next().unwrap();

    let mut at_position: (usize, usize) = (0, 0);

    let parsed_map: Vec<Vec<char>> = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        at_position = (x, y);
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let parsed_moves: Vec<char> = moves
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    (parsed_map, parsed_moves, at_position)
}

fn resize_map(original_map: &[Vec<char>]) -> Vec<Vec<char>> {
    original_map
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|&tile| match tile {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<char>>()
        })
        .collect()
}

#[macroquad::main("Rust Macroquad Grid Canvas")]
async fn main() {
    let filename = advent_of_code_2024::load_input();
    let (mut map, moves, mut pos) = parse_input(&filename);

    map = resize_map(&map);
    pos.0 *= 2;
    let mut robot = Robot::new(pos.0 as i32, pos.1 as i32);

    for &m in &moves {
        let dir = match m {
            '>' => Direction::Right,
            '^' => Direction::Top,
            '<' => Direction::Left,
            'v' => Direction::Bottom,
            _ => unreachable!(),
        };

        robot.advance(dir, &mut map);

        // render the updated map after each move
        let (width, height) = (screen_width(), screen_height());
        let cell_size_x = (width / map[0].len() as f32).floor() as i32;
        let cell_size_y = (height / map.len() as f32).floor() as i32;

        clear_background(WHITE);

        // draw the map and grid
        draw_map(&map, cell_size_x, cell_size_y);
        draw_grid(
            map[0].len() as i32,
            map.len() as i32,
            cell_size_x,
            cell_size_y,
        );

        next_frame().await;

        sleep(Duration::from_millis(10));
    }

    // wait for the player to press `q` before exiting
    loop {
        clear_background(WHITE);

        let (width, height) = (screen_width(), screen_height());
        let cell_size_x = (width / map[0].len() as f32).floor() as i32;
        let cell_size_y = (height / map.len() as f32).floor() as i32;

        draw_map(&map, cell_size_x, cell_size_y);
        draw_grid(
            map[0].len() as i32,
            map.len() as i32,
            cell_size_x,
            cell_size_y,
        );

        draw_text("Press 'q' to quit", 10.0, screen_height() - 20.0, 30.0, RED);

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }

    let output = calculte_total(&map);
    println!("{output}");
}

fn draw_map(map: &[Vec<char>], cell_size_x: i32, cell_size_y: i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let screen_x = x as f32 * cell_size_x as f32;
            let screen_y = y as f32 * cell_size_y as f32;

            match c {
                '#' => draw_rectangle(
                    screen_x,
                    screen_y,
                    cell_size_x as f32,
                    cell_size_y as f32,
                    GRAY,
                ),
                '.' => draw_rectangle(
                    screen_x,
                    screen_y,
                    cell_size_x as f32,
                    cell_size_y as f32,
                    WHITE,
                ),
                '@' => draw_rectangle(
                    screen_x,
                    screen_y,
                    cell_size_x as f32,
                    cell_size_y as f32,
                    BLUE,
                ),
                '[' => {
                    if let Some(next_char) = row.get(x + 1) {
                        if *next_char == ']' {
                            // Draw a green rectangle for the box
                            draw_rectangle(
                                screen_x,
                                screen_y,
                                (cell_size_x * 2) as f32,
                                (cell_size_y * 2) as f32,
                                BROWN,
                            );
                            // Draw a black border around the box
                            draw_rectangle_lines(
                                screen_x,
                                screen_y,
                                (cell_size_x * 2) as f32,
                                (cell_size_y * 2) as f32,
                                5.0,
                                BLACK,
                            );
                        } else {
                            draw_rectangle(
                                screen_x,
                                screen_y,
                                cell_size_x as f32,
                                cell_size_y as f32,
                                BROWN,
                            );
                        }
                    }
                }
                ']' => {
                    // If we are at the end of a `[]` box, do nothing because it's handled by the opening `[`
                }
                _ => {}
            }
        }
    }
}

fn draw_grid(cols: i32, rows: i32, cell_size_x: i32, cell_size_y: i32) {
    for x in 0..=cols {
        let start_x = x * cell_size_x;
        draw_line(
            start_x as f32,
            0.0,
            start_x as f32,
            rows as f32 * cell_size_y as f32,
            1.0,
            DARKGRAY,
        );
    }
    for y in 0..=rows {
        let start_y = y * cell_size_y;
        draw_line(
            0.0,
            start_y as f32,
            cols as f32 * cell_size_x as f32,
            start_y as f32,
            1.0,
            DARKGRAY,
        );
    }
}

fn calculte_total(map: &[Vec<char>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| if c == '[' { 100 * y + x } else { 0 })
        })
        .sum()
}
