// 6.17 BFS

use std::collections::{HashMap, VecDeque};

use thiserror::Error;

use crate::{graph::Graph, tasks::task2::get_incoming_nodes};

#[derive(Debug, Clone, Error)]
pub enum IsAcyclicError {
    #[error("Graph is undirected")]
    UndirectedGraph,
}

pub fn is_acyclic(graph: &Graph) -> Result<bool, IsAcyclicError> {
    if !graph.directed {
        return Err(IsAcyclicError::UndirectedGraph);
    }

    let mut nodes_in_degree: HashMap<usize, usize> = graph
        .edges
        .keys()
        .map(|node| (*node, get_incoming_nodes(graph, *node).unwrap().len()))
        .collect();

    let mut queue = VecDeque::new();
    let mut visited_count: usize = 0;

    for (node, in_degree) in nodes_in_degree.iter() {
        if *in_degree == 0 {
            queue.push_back(*node);
        }
    }

    while let Some(node) = queue.pop_front() {
        visited_count += 1;
        for neighbour in graph.edges.get(&node).unwrap() {
            let in_degree = nodes_in_degree.get_mut(&neighbour.node).unwrap();
            *in_degree -= 1;
            if *in_degree == 0 {
                queue.push_back(neighbour.node);
            }
        }
    }

    Ok(visited_count == graph.edges.len())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maplit::{hashmap, hashset};

    use crate::graph::Edge;

    use super::*;

    #[test]
    fn test_is_acyclic_for_acyclic_graph() {
        let mut graph = Graph::new(true);

        graph.edges = hashmap! {
            1 => hashset! { Edge { node: 3 } },
            2 => hashset! { Edge { node: 3 } },
            3 => hashset! { Edge { node: 4 }, Edge { node: 5 } },
            4 => hashset! { Edge { node: 5 } },
            5 => HashSet::new()
        };

        assert!(is_acyclic(&graph).unwrap());
    }

    #[test]
    fn test_is_acyclic_for_cyclic_graph() {
        let mut graph = Graph::new(true);

        graph.edges = hashmap! {
            0 => hashset! { Edge { node: 1 } },
            1 => hashset! { Edge { node: 2 } },
            2 => hashset! { Edge { node: 3 } },
            3 => hashset! { Edge { node: 1 } },
        };

        assert!(!is_acyclic(&graph).unwrap());
    }
}
