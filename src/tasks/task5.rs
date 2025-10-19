// 5.13 DFS

use std::collections::{HashMap, HashSet};

use crate::graph::{Edge, Graph};

fn transpose(graph: &mut Graph) {
    graph.edges = graph
        .edges
        .iter()
        .map(|(node, neighbours)| {
            neighbours
                .iter()
                .map(|neighbour| (*node, neighbour.clone()))
        })
        .flatten()
        .map(|(node, neighbour)| (neighbour.node, node))
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key)
                .or_insert_with(HashSet::new)
                .insert(Edge { node: value });
            acc
        });
}

#[cfg(test)]
mod tests {
    use maplit::{hashmap, hashset};

    use super::*;

    #[test]
    fn test_transpose() {
        let mut graph = Graph::new(true);

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(0, 1).unwrap();
        graph.add_edge(0, 2).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 1).unwrap();

        transpose(&mut graph);

        assert_eq!(
            graph.edges,
            hashmap! {
                1 => hashset! {Edge { node: 0 }, Edge { node: 2 }},
                2 => hashset! {Edge { node: 0 }, Edge { node: 1 }}
            }
        )
    }
}
