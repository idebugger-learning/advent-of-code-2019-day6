use petgraph::prelude::*;

pub fn parse(input: &str) -> GraphMap<&str, (), Undirected> {
    let mut graph = GraphMap::<&str, (), Undirected>::new();

    input
        .split('\n')
        .map(|row| row.split(')').collect::<Vec<_>>())
        .for_each(|row| {
            let from = row[0];
            let to = row[1];

            let node_from = graph.add_node(from);
            let node_to = graph.add_node(to);

            graph.add_edge(node_from, node_to, ());
        });

    graph
}
