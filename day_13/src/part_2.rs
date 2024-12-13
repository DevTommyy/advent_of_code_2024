use std::fs;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
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
            let data: Vec<Vec<i64>> = game
                .lines()
                .map(|line| {
                    line.split(": ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .flat_map(|part| {
                            let parts: Vec<i64> = part[2..]
                                .split('+')
                                .map(|val| val.parse().unwrap())
                                .collect();
                            parts
                        })
                        .collect::<Vec<i64>>()
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
                    x: data[2][0] + 10_000_000_000_000,
                    y: data[2][1] + 10_000_000_000_000,
                },
            }
        })
        .collect()
}

fn calculate_token(game: &Game) -> Option<i64> {
    let denominator = (game.a.x as f64 * game.b.y as f64) - (game.a.y as f64 * game.b.x as f64);

    let delta_a = ((game.prize.x as f64 * game.b.y as f64)
        - (game.prize.y as f64 * game.b.x as f64))
        / denominator;
    let delta_b = ((game.prize.x as f64) - ((game.a.x as f64) * delta_a)) / (game.b.x as f64);

    if delta_a.trunc() == delta_a && delta_b.trunc() == delta_b {
        let result = (delta_a * 3.0 + delta_b) as i64;
        Some(result)
    } else {
        None
    }
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let input = parse_input(&filename);

    let output: i64 = input.iter().flat_map(calculate_token).sum();
    println!("{output}");
}
