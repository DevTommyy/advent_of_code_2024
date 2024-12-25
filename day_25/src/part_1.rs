use std::fs;

fn parse_input(filename: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, usize) {
    let content = fs::read_to_string(filename).expect("failed to read file");
    let grids = content.split("\n\n").collect::<Vec<_>>();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut total_height = 0;
    for g in grids.iter() {
        let g = g
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let is_key = g[0][0] != '#';
        total_height = g.len();

        let mut heights = vec![0usize; g[0].len()];
        for row in g {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    heights[x] += 1;
                }
            }
        }

        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    (locks, keys, total_height)
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (locks, keys, total_height) = parse_input(&filename);

    let mut output = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock).all(|(a, b)| a + b <= total_height) {
                output += 1;
            }
        }
    }

    println!("{output}");
}
