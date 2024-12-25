use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_input(filename: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let input = fs::read_to_string(filename).expect("Could not read file");
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let wires = wires
        .lines()
        .map(|w| {
            let s = w.split_once(": ").unwrap();
            (s.0.to_string(), s.1 == "1")
        })
        .collect::<HashMap<_, _>>();

    let gates = gates
        .lines()
        .map(|l| {
            let (left, out) = l.split_once(" -> ").unwrap();
            let s = left.split_whitespace().collect::<Vec<_>>();
            let logic = match s[1] {
                "AND" => Logic::And,
                "OR" => Logic::Or,
                "XOR" => Logic::Xor,
                _ => unreachable!(),
            };
            Gate {
                logic,
                a: s[0].to_string(),
                b: s[2].to_string(),
                out: out.to_string(),
            }
        })
        .collect::<Vec<_>>();

    (wires, gates)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    logic: Logic,
    a: String,
    b: String,
    out: String,
}

fn get_value(wires: &HashMap<String, bool>, prefix: &str) -> u64 {
    let mut wires_to_decode = wires
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect::<Vec<_>>();

    wires_to_decode.sort_by_key(|(name, _)| (*name).clone());
    wires_to_decode.reverse();

    let mut result = 0u64;
    for (_, v) in wires_to_decode {
        result <<= 1;
        if *v {
            result += 1;
        }
    }

    result
}

fn run(wires: &HashMap<String, bool>, gates: &[Gate], renames: &HashMap<String, String>) -> u64 {
    let mut wires = wires.clone();
    let mut queue = VecDeque::from(gates.to_vec());

    while let Some(g) = queue.pop_front() {
        let out = renames.get(&g.out).unwrap_or(&g.out).clone();

        let a = match wires.get(&g.a) {
            Some(&val) => val,
            None => {
                queue.push_back(g);
                continue;
            }
        };

        let b = match wires.get(&g.b) {
            Some(&val) => val,
            None => {
                queue.push_back(g);
                continue;
            }
        };

        let v = match g.logic {
            Logic::And => a && b,
            Logic::Or => a || b,
            Logic::Xor => a != b,
        };

        wires.insert(out, v);
    }

    get_value(&wires, "z")
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let (wires, gates) = parse_input(&filename);

    let output = run(&wires, &gates, &HashMap::new());
    println!("{output}");
}
