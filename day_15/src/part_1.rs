use std::fs;

fn try_move(pos: &mut (usize, usize), next_move: char, map: &mut [Vec<char>]) {
    let direction: (isize, isize) = match next_move {
        '<' => (-1, 0),
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        _ => unreachable!(),
    };

    let mut current_x = pos.0 as isize;
    let mut current_y = pos.1 as isize;
    let mut can_move = true;
    let mut boxes_to_move = Vec::new();

    loop {
        let next_x = (current_x + direction.0) as usize;
        let next_y = (current_y + direction.1) as usize;

        match map[next_y][next_x] {
            '#' => {
                can_move = false;
                break;
            }
            'O' => {
                boxes_to_move.push((next_x, next_y));
                current_x += direction.0;
                current_y += direction.1;
            }
            '.' => {
                break;
            }
            _ => unreachable!(),
        }
    }

    if can_move {
        for &(box_x, box_y) in boxes_to_move.iter().rev() {
            let next_x = (box_x as isize + direction.0) as usize;
            let next_y = (box_y as isize + direction.1) as usize;
            map[next_y][next_x] = 'O';
            map[box_y][box_x] = '.';
        }

        let new_x = (pos.0 as isize + direction.0) as usize;
        let new_y = (pos.1 as isize + direction.1) as usize;
        map[new_y][new_x] = '@';
        map[pos.1][pos.0] = '.';
        pos.0 = new_x;
        pos.1 = new_y;
    }
}

fn calculte_total(map: &[Vec<char>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, elem)| **elem == 'O')
                .map(|(x, _circle)| y * 100 + x)
                .sum::<usize>()
        })
        .sum()
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

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (mut map, moves, mut pos) = parse_input(&filename);

    moves
        .iter()
        .for_each(|&next_move| try_move(&mut pos, next_move, &mut map));

    // map.iter()
    //     .for_each(|line| println!("{}", line.iter().collect::<String>()));

    let output = calculte_total(&map);
    println!("{output}");
}
