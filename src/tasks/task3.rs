use std::collections::HashSet;

use thiserror::Error;

use crate::{
    graph::Graph,
    tasks::task2::{OutgoingNodesError, get_outgoing_nodes},
};

#[derive(Clone, Debug, Error)]
pub enum NodesWithGreaterOutdegreeError {
    #[error("Node does not exist")]
    NodeDoesNotExist,

    #[error("Graph is undirected")]
    UndirectedGraph,
}

pub fn get_nodes_with_greater_outdegree(
    graph: &Graph,
    node: usize,
) -> Result<HashSet<usize>, NodesWithGreaterOutdegreeError> {
    match get_outgoing_nodes(graph, node) {
        Ok(outgoing_nodes) => Ok(graph
            .edges
            .keys()
            .map(|k| *k)
            .filter(|k| {
                *k != node && get_outgoing_nodes(graph, node).unwrap().len() > outgoing_nodes.len()
            })
            .collect()),
        Err(e) => match e {
            OutgoingNodesError::NodeDoesNotExist => {
                Err(NodesWithGreaterOutdegreeError::NodeDoesNotExist)
            }
            OutgoingNodesError::UndirectedGraph => {
                Err(NodesWithGreaterOutdegreeError::UndirectedGraph)
            }
        },
    }
}
