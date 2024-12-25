use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let connections = parse_input(&filename);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // make connections into an undirected graph
    for connection in &connections {
        let parts: Vec<&str> = connection.split('-').collect();
        let (a, b) = (parts[0], parts[1]);

        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut triplets = HashSet::new();

    // find sets of 3 connected computers
    for (&node, neighbors) in &graph {
        let neighbors: Vec<&&str> = neighbors.iter().collect();
        for i in 0..neighbors.len() {
            for j in i + 1..neighbors.len() {
                let a = neighbors[i];
                let b = neighbors[j];

                if graph.get(a).map_or(false, |set| set.contains(b)) {
                    let mut triplet = vec![node, *a, *b];
                    // sort them so its uique
                    triplet.sort();
                    triplets.insert(triplet);
                }
            }
        }
    }

    let output = triplets
        .iter()
        .filter(|triplet| triplet.iter().any(|&comp| comp.starts_with('t')))
        .count();

    println!("{output}");
}
