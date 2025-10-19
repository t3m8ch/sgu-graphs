use std::collections::HashSet;

use thiserror::Error;

use crate::graph::{Edge, Graph};

#[derive(Clone, Debug, Error)]
pub enum OutgoingNodesError {
    #[error("Node does not exist")]
    NodeDoesNotExist,

    #[error("Graph is undirected")]
    UndirectedGraph,
}

pub fn get_outgoing_nodes(graph: &Graph, node: usize) -> Result<Vec<usize>, OutgoingNodesError> {
    if !graph.directed {
        return Err(OutgoingNodesError::UndirectedGraph);
    }

    if !graph.contains_node(node) {
        return Err(OutgoingNodesError::NodeDoesNotExist);
    }

    Ok(graph
        .edges
        .get(&node)
        .unwrap_or(&HashSet::new())
        .iter()
        .map(|e| e.node)
        .collect())
}

#[derive(Clone, Debug, Error)]
pub enum IncomingNodesError {
    #[error("Node does not exist")]
    NodeDoesNotExist,

    #[error("Graph is undirected")]
    UndirectedGraph,
}

pub fn get_incoming_nodes(graph: &Graph, node: usize) -> Result<Vec<usize>, IncomingNodesError> {
    if !graph.directed {
        return Err(IncomingNodesError::UndirectedGraph);
    }

    if !graph.contains_node(node) {
        return Err(IncomingNodesError::NodeDoesNotExist);
    }

    Ok(graph
        .edges
        .iter()
        .filter_map(|(from_node, to_nodes)| {
            if to_nodes.contains(&Edge { node }) {
                Some(*from_node)
            } else {
                None
            }
        })
        .collect())
}
