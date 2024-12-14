use image::{GrayImage, Luma};
use std::fs;

const BOUND_X: isize = 101; // 11 - 101
const BOUND_Y: isize = 103; // 7 - 103

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Velocity,
}

impl Robot {
    fn from(position: (isize, isize), velocity: (isize, isize)) -> Self {
        Robot {
            position: Point::from(position),
            velocity: Velocity::from(velocity),
        }
    }

    fn advance(&mut self) {
        let mut new_x = self.position.x + self.velocity.dx;
        let mut new_y = self.position.y + self.velocity.dy;

        // pacman effect for x
        if new_x < 0 {
            new_x = (new_x + BOUND_X).rem_euclid(BOUND_X);
        } else if new_x >= BOUND_X {
            new_x = new_x.rem_euclid(BOUND_X);
        }

        // pacman effect for y
        if new_y < 0 {
            new_y = (new_y + BOUND_Y).rem_euclid(BOUND_Y);
        } else if new_y >= BOUND_Y {
            new_y = new_y.rem_euclid(BOUND_Y);
        }

        self.position = Point::new(new_x, new_y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    // assuming position is x, y
    fn from(position: (isize, isize)) -> Self {
        Point {
            x: position.0,
            y: position.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    dx: isize,
    dy: isize,
}

impl Velocity {
    // assuming velocity is dx, dy
    fn from(velocity: (isize, isize)) -> Self {
        Velocity {
            dx: velocity.0,
            dy: velocity.1,
        }
    }
}

fn parse_input(filename: &str) -> Vec<Robot> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();

            let parse_coords = |part: &str| {
                let coords: Vec<isize> = part[2..]
                    .split(',')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect();
                (coords[0], coords[1])
            };

            let position = parse_coords(parts[0]);
            let velocity = parse_coords(parts[1]);

            Robot::from(position, velocity)
        })
        .collect()
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let mut input = parse_input(&filename);

    for i in 0..10000 {
        let mut img = GrayImage::new(BOUND_X as u32, BOUND_Y as u32);
        input.iter_mut().for_each(|robot| robot.advance());

        for &robot in input.iter() {
            let x = robot.position.x as u32;
            let y = robot.position.y as u32;

            img.put_pixel(x, y, Luma([255]));
        }
        img.save(format!("output/output{}.png", i + 1)).unwrap();
    }
}
