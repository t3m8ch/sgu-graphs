use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use thiserror::Error;

use crate::graph::Graph;

#[derive(Debug, Clone, Error)]
pub enum MstPrimError {
    #[error("Start node does not exist")]
    StartNodeDoesNotExist,

    #[error("Graph must be undirected")]
    DirectedGraph,
}

pub fn mst_prim(graph: &Graph, start_node: usize) -> Result<Graph, MstPrimError> {
    if graph.directed {
        return Err(MstPrimError::DirectedGraph);
    }

    if !graph.contains_node(start_node) {
        return Err(MstPrimError::StartNodeDoesNotExist);
    }

    let mut visited = HashSet::new();
    let mut mst = Graph::new(false);
    let mut queue = BinaryHeap::new();

    visited.insert(start_node);
    for neighbour in graph.edges.get(&start_node).unwrap() {
        queue.push(Reverse((neighbour.weight, start_node, neighbour.node)));
    }

    while let Some(Reverse((weight, from, to))) = queue.pop()
        && visited.len() < graph.edges.len()
    {
        if visited.contains(&to) {
            continue;
        }

        mst.add_node(from);
        mst.add_node(to);
        mst.add_rib(from, to, weight, 1).unwrap();
        visited.insert(to);

        for neighbour in graph.edges.get(&to).unwrap() {
            if !visited.contains(&neighbour.node) {
                queue.push(Reverse((neighbour.weight, to, neighbour.node)));
            }
        }
    }

    Ok(mst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mst_prim() {
        let mut graph = Graph::new(false);

        for i in 1..=6 {
            graph.add_node(i);
        }

        graph.add_rib(1, 2, 4, 1).unwrap();
        graph.add_rib(1, 3, 2, 1).unwrap();
        graph.add_rib(2, 3, 1, 1).unwrap();
        graph.add_rib(2, 4, 5, 1).unwrap();
        graph.add_rib(3, 4, 8, 1).unwrap();
        graph.add_rib(3, 5, 10, 1).unwrap();
        graph.add_rib(4, 5, 2, 1).unwrap();
        graph.add_rib(4, 6, 6, 1).unwrap();
        graph.add_rib(5, 6, 3, 1).unwrap();

        let mut expected_mst = Graph::new(false);
        for i in 1..=6 {
            expected_mst.add_node(i);
        }
        expected_mst.add_rib(1, 3, 2, 1).unwrap();
        expected_mst.add_rib(3, 2, 1, 1).unwrap();
        expected_mst.add_rib(2, 4, 5, 1).unwrap();
        expected_mst.add_rib(4, 5, 2, 1).unwrap();
        expected_mst.add_rib(5, 6, 3, 1).unwrap();

        let actual_mst = mst_prim(&graph, 1).unwrap();

        assert_eq!(expected_mst.edges, actual_mst.edges);
    }
}
