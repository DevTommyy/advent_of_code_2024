use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_input(filename: &str) -> Vec<Game> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .split("\n\n")
        .map(|game| {
            let data: Vec<Vec<i32>> = game
                .lines()
                .map(|line| {
                    line.split(": ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .flat_map(|part| {
                            let parts: Vec<i32> = part[2..]
                                .split('+')
                                .map(|val| val.parse().unwrap())
                                .collect();
                            parts
                        })
                        .collect::<Vec<i32>>()
                })
                .collect();

            Game {
                a: Point {
                    x: data[0][0],
                    y: data[0][1],
                },
                b: Point {
                    x: data[1][0],
                    y: data[1][1],
                },
                prize: Point {
                    x: data[2][0],
                    y: data[2][1],
                },
            }
        })
        .collect()
}

fn calculate_token(data: &Game) -> i32 {
    (0..=100)
        .flat_map(|a| {
            (0..=100).filter_map(move |b| {
                if (data.a.x * a + data.b.x * b) == data.prize.x
                    && (data.a.y * a + data.b.y * b) == data.prize.y
                {
                    Some(a * 3 + b)
                } else {
                    None
                }
            })
        })
        .min()
        .unwrap_or(0)
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let output: i32 = input.iter().map(calculate_token).sum();
    println!("{output}");
}
