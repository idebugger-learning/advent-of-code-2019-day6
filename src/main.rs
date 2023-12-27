use crate::parser::parse;
use petgraph::algo::all_simple_paths;

mod parser;

fn main() {
    // let input = include_str!("../data/example.txt");
    let input = include_str!("../data/input.txt");

    let graph = parse(input);
    let from = graph
        .nodes()
        .find(|name| name == &"YOU")
        .expect("From node not found");
    let to = graph
        .nodes()
        .find(|name| name == &"SAN")
        .expect("To node not found");

    let paths = all_simple_paths::<Vec<_>, _>(&graph, from, to, 0, None).collect::<Vec<_>>();

    println!("Paths: {:?}", paths);
    // println!("Sum of paths: {}", paths.values().sum::<usize>());
    println!("Path to SAN: {:?}", paths.get(0).unwrap().len() - 3);
}
