use pathfinding::prelude::count_paths;
use std::collections::HashMap;

const START_NODE: &str = "you";
const END_NODE: &str = "out";

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let empty: Vec<String> = Vec::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (key, vals) = line.split_once(":").unwrap();
        let node = key.to_string();
        let connections: Vec<String> = vals.split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node, connections);
    }

    println!("{:?}", graph);
    let paths = count_paths(
        START_NODE.to_string(),
        |n: &String| graph.get(n).unwrap_or(&empty).iter().cloned(),
        |n: &String| *n == END_NODE,
    );
    println!("{:?}", paths);
}
