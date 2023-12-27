use crate::parser::parse;
use petgraph::prelude::*;
use std::collections::HashMap;

mod parser;

fn main() {
    // let input = include_str!("../data/example.txt");
    let input = include_str!("../data/input.txt");

    let graph = parse(input);
    let first_node = graph
        .nodes()
        .find(|name| name == &"COM")
        .expect("First node not found");

    let mut dfs = Dfs::new(&graph, first_node);
    let mut paths = HashMap::<&str, usize>::new();
    paths.insert(first_node, 0);
    while let Some(node) = dfs.next(&graph) {
        if node == "COM" {
            continue;
        }

        let (from, _, _) = graph
            .edges_directed(node, Incoming)
            .next()
            .expect("Edge not found");
        let path_from = paths.get(&from).expect("Path for parent not found");
        paths.insert(node, path_from + 1);
    }

    println!("Paths: {:?}", paths);
    println!("Sum of paths: {}", paths.values().sum::<usize>());
}
