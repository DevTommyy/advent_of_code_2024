use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

// basically find all the groups via graph traversal
fn find_groups<'a>(
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
    potential_group: HashSet<&'a str>,
    candidates: HashSet<&'a str>,
    mut excluded: HashSet<&'a str>,
) -> Vec<HashSet<&'a str>> {
    if candidates.is_empty() && excluded.is_empty() {
        return vec![potential_group];
    }

    let mut groups = Vec::new();
    let mut candidates = candidates;

    while let Some(&node) = candidates.iter().next() {
        let mut new_clique = potential_group.clone();
        new_clique.insert(node);

        let neighbors: HashSet<_> = graph.get(node).unwrap_or(&HashSet::new()).clone();
        let new_candidates: HashSet<_> = candidates.intersection(&neighbors).cloned().collect();
        let new_excluded: HashSet<_> = excluded.intersection(&neighbors).cloned().collect();

        groups.extend(find_groups(graph, new_clique, new_candidates, new_excluded));

        candidates.remove(node);
        excluded.insert(node);
    }

    groups
}

fn main() {
    let filename = advent_of_code_2024::load_input();
    let connections = parse_input(&filename);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // make connections into an undirected graph
    for connection in &connections {
        let parts: Vec<&str> = connection.split('-').collect();
        let (a, b) = (parts[0], parts[1]);

        // connect both ways
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let all_nodes: HashSet<_> = graph.keys().cloned().collect();
    let groups = find_groups(&graph, HashSet::new(), all_nodes, HashSet::new());

    let largest_group = groups
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap_or_default();

    let mut password: Vec<&str> = largest_group.into_iter().collect();
    password.sort();

    let output = password.join(",");
    println!("{output}");
}
