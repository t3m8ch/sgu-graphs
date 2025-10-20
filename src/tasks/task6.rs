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

    // TODO: Учитывать изолированные вершины и другие компоненты связности
    // (мы должны проверять все компоненты, есть ли в них циклы)

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
            1 => hashset! { Edge::value(3).build() },
            2 => hashset! { Edge::value(3).build() },
            3 => hashset! { Edge::value(4).build(), Edge::value(5).build() },
            4 => hashset! { Edge::value(5).build() },
            5 => HashSet::new()
        };

        assert!(is_acyclic(&graph).unwrap());
    }

    #[test]
    fn test_is_acyclic_for_cyclic_graph() {
        let mut graph = Graph::new(true);

        graph.edges = hashmap! {
            0 => hashset! { Edge::value(1).build() },
            1 => hashset! { Edge::value(2).build() },
            2 => hashset! { Edge::value(3).build() },
            3 => hashset! { Edge::value(1).build() },
        };

        assert!(!is_acyclic(&graph).unwrap());
    }
}
