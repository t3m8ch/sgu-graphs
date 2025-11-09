// Алгоритм Беллмана-Форда

use std::collections::HashMap;

use itertools::process_results;
use thiserror::Error;

use crate::graph::Graph;

#[derive(Debug, Clone, Error)]
pub enum MinDistanceError {
    #[error("Negative cycle detected")]
    NegativeCycle,

    #[error("Empty graph")]
    EmptyGraph,
}

pub fn min_distance(graph: &Graph) -> Result<(usize, i32), MinDistanceError> {
    let results = graph
        .edges
        .keys()
        .map(|node| (*node, bellman_ford(graph, *node)))
        .map(|(node, distances)| distances.map(|d| (node, d.values().sum())));

    process_results(results, |iter| iter.min_by_key(|(_, sum)| *sum))?
        .ok_or(MinDistanceError::EmptyGraph)
}

fn bellman_ford(graph: &Graph, source: usize) -> Result<HashMap<usize, i32>, MinDistanceError> {
    let mut distances = HashMap::new();
    distances.insert(source, 0);

    let edges = graph
        .edges
        .iter()
        .map(|(from_node, to_nodes)| {
            to_nodes
                .iter()
                .map(move |to| (from_node, &to.node, &to.weight))
        })
        .flatten();

    for _ in 0..graph.edges.len() {
        for (from, to, weight) in edges.clone() {
            let Some(from_dist) = distances.get(from) else {
                continue;
            };

            let new_dist = from_dist + weight;
            let curr_dist = *distances.get(to).unwrap_or(&i32::MAX);
            if new_dist < curr_dist {
                distances.insert(*to, new_dist);
            }
        }
    }

    for (from, to, weight) in edges.clone() {
        let Some(from_dist) = distances.get(from) else {
            continue;
        };

        let new_dist = from_dist + weight;
        let curr_dist = *distances.get(to).unwrap_or(&i32::MAX);
        if new_dist < curr_dist {
            return Err(MinDistanceError::NegativeCycle);
        }
    }

    Ok(distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        // Тестовые данные взяты из статьи:
        // https://habr.com/ru/companies/otus/articles/484382/
        let mut graph = Graph::new(true);

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);

        graph.add_edge(0, 1, -1).unwrap();
        graph.add_edge(0, 2, 4).unwrap();
        graph.add_edge(1, 2, 3).unwrap();
        graph.add_edge(1, 3, 2).unwrap();
        graph.add_edge(1, 4, 2).unwrap();
        graph.add_edge(3, 2, 5).unwrap();
        graph.add_edge(3, 1, 1).unwrap();
        graph.add_edge(4, 3, -3).unwrap();

        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result[&0], 0);
        assert_eq!(result[&1], -1);
        assert_eq!(result[&2], 2);
        assert_eq!(result[&3], -2);
        assert_eq!(result[&4], 1);
    }
}
