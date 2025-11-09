use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use thiserror::Error;

use crate::graph::Graph;

#[derive(Debug, Clone, Error)]
pub enum MinimalLengthToNodesError {
    #[error("Negative weight")]
    NegativeWeight { from: usize, to: usize, weight: i32 },
}

// Алгоритм Дейкстры
pub fn minimal_length_to_nodes(
    graph: &Graph,
    source: usize,
) -> Result<HashMap<usize, u32>, MinimalLengthToNodesError> {
    let mut distances = HashMap::new();
    distances.insert(source, 0);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, source)));

    while let Some(Reverse((distance, node))) = queue.pop() {
        if distance > *distances.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        for neighbor in graph.edges[&node].iter() {
            if neighbor.weight < 0 {
                return Err(MinimalLengthToNodesError::NegativeWeight {
                    from: node,
                    to: neighbor.node,
                    weight: neighbor.weight,
                });
            }

            let curr_dist = *distances.get(&neighbor.node).unwrap_or(&u32::MAX);
            let new_dist = *distances.get(&node).unwrap_or(&u32::MAX) + neighbor.weight as u32;

            if curr_dist > new_dist {
                distances.insert(neighbor.node, new_dist);
                queue.push(Reverse((new_dist, neighbor.node)));
            }
        }
    }

    Ok(distances)
}
