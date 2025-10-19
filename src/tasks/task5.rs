// 5.13 DFS

use std::collections::{HashMap, HashSet};

use bon::builder;

use crate::graph::{Edge, Graph};

pub fn scc_count(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for node in graph.edges.keys() {
        if !visited.contains(node) {
            dfs()
                .graph(graph)
                .source(*node)
                .visited(&mut visited)
                .stack(&mut stack)
                .call();
        }
    }

    let mut reversed_graph = graph.clone();
    transpose(&mut reversed_graph);

    let mut visited = HashSet::new();
    let mut component_count = 0;

    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            dfs()
                .graph(&reversed_graph)
                .source(node)
                .visited(&mut visited)
                .call();

            component_count += 1;
        }
    }

    component_count
}

#[builder]
fn dfs(
    graph: &Graph,
    source: usize,
    visited: &mut HashSet<usize>,
    mut stack: Option<&mut Vec<usize>>,
) {
    visited.insert(source);

    for neighbour in graph.edges.get(&source).unwrap() {
        if !visited.contains(&neighbour.node) {
            dfs()
                .graph(graph)
                .source(neighbour.node)
                .visited(visited)
                .maybe_stack(stack.as_deref_mut())
                .call();
        }
    }

    if let Some(stack) = stack {
        stack.push(source);
    }
}

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

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(true);

        graph.edges = hashmap! {
            0 => hashset! { Edge { node: 1 }, Edge { node: 2 }, Edge { node: 3 } },
            1 => hashset! { Edge { node: 3 }, Edge { node: 4 } },
            2 => hashset! { Edge { node: 3 } },
            3 => hashset! { Edge { node: 1 }, Edge { node: 4 } },
            4 => HashSet::new(),
            5 => HashSet::new(),
        };

        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        dfs()
            .graph(&graph)
            .source(0)
            .visited(&mut visited)
            .stack(&mut stack)
            .call();

        assert_eq!(visited, hashset! {0, 1, 3, 4, 2});
        assert!(stack == vec![4, 3, 1, 2, 0] || stack == vec![4, 1, 3, 2, 0]);
    }

    #[test]
    fn test_scc_count() {
        let mut graph = Graph::new(true);

        graph.edges = hashmap! {
            0 => hashset! { Edge { node: 1 } },
            1 => hashset! { Edge { node: 2 } },
            2 => hashset! { Edge { node: 0 } },
            3 => hashset! { Edge { node: 2 }, Edge { node: 4 } },
            4 => hashset! { Edge { node: 7 }, Edge { node: 3 } },
            5 => hashset! { Edge { node: 7 } },
            6 => hashset! { Edge { node: 5 }, Edge { node: 8 } },
            7 => hashset! { Edge { node: 6 } },
            8 => hashset! { Edge { node: 7 } },
        };

        assert_eq!(scc_count(&graph), 3);
    }
}
