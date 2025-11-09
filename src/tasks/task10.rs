use std::collections::{HashMap, HashSet};

use crate::graph::Graph;

pub fn n_periphery(graph: &Graph, target: usize, n: i32) -> HashSet<usize> {
    floyd_warshall(graph)
        .into_iter()
        .filter_map(|(from, to, distance)| {
            (to == target && distance.map_or(true, |d| d > n)).then_some(from)
        })
        .collect()
}

fn floyd_warshall(graph: &Graph) -> Vec<(usize, usize, Option<i32>)> {
    let nodes: Vec<usize> = graph.edges.keys().copied().collect();
    let n = nodes.len();

    if n == 0 {
        return Vec::new();
    }

    let node_to_idx: HashMap<usize, usize> = nodes
        .iter()
        .enumerate()
        .map(|(idx, &node)| (node, idx))
        .collect();

    const INF: i32 = i32::MAX / 2;
    let mut dist = vec![vec![INF; n]; n];

    for i in 0..n {
        dist[i][i] = 0;
    }

    for (&from, edges) in &graph.edges {
        let i = node_to_idx[&from];
        for edge in edges {
            let j = node_to_idx[&edge.node];
            dist[i][j] = edge.weight;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != INF && dist[k][j] != INF {
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist < dist[i][j] {
                        dist[i][j] = new_dist;
                    }
                }
            }
        }
    }

    let mut result = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            let distance = if dist[i][j] == INF {
                None
            } else {
                Some(dist[i][j])
            };
            result.push((nodes[i], nodes[j], distance));
        }
    }

    result
}
