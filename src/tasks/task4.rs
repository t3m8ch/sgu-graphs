use thiserror::Error;

use crate::graph::Graph;

#[derive(Clone, Debug, Error)]
pub enum SymDiffError {
    #[error("First graph is undirected")]
    FirstUndirectedGraph,

    #[error("Second graph is undirected")]
    SecondUndirectedGraph,
}

pub fn sym_diff(first: &Graph, second: &Graph) -> Result<Graph, SymDiffError> {
    if !first.directed {
        return Err(SymDiffError::FirstUndirectedGraph);
    }
    if !second.directed {
        return Err(SymDiffError::SecondUndirectedGraph);
    }

    let edges = first
        .edges
        .iter()
        .filter_map(|(k, v1)| {
            second
                .edges
                .get(&k)
                .map(|v2| (*k, v1.symmetric_difference(v2).cloned().collect()))
        })
        .collect();

    Ok(Graph {
        edges,
        directed: true,
    })
}
