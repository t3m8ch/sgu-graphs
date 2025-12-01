use std::{
    collections::{HashMap, HashSet, VecDeque},
    i32,
};

use crate::graph::Graph;

pub fn ford_fulkerson(graph: &Graph, source: usize, sink: usize) -> i32 {
    let mut flow: HashMap<(usize, usize), i32> = HashMap::new();

    let capacity: HashMap<(usize, usize), i32> = graph
        .edges
        .iter()
        .map(|(u, vs)| vs.iter().map(|v| ((*u, v.node), v.capacity)))
        .flatten()
        .collect();

    let mut max_flow = 0;

    loop {
        // Поиск увеличивающегося пути при помощи BFS
        let parent = bfs(graph, source, sink, &flow, &capacity);

        // Если путь до стока не найден - завершаем
        if !parent.contains_key(&sink) {
            break;
        }

        // Находим минимальную остаточную пропускную способность
        let mut path_flow = i32::MAX;
        let mut v = sink;
        while v != source {
            let u = parent[&v];
            let residual = get_residual_capacity(u, v, &capacity, &flow);
            path_flow = path_flow.min(residual);
            v = u;
        }

        // Обновляем поток вдоль найденного пути
        v = sink;
        while v != source {
            let u = parent[&v];
            *flow.entry((u, v)).or_default() += path_flow;
            *flow.entry((v, u)).or_default() -= path_flow;
            v = u;
        }

        max_flow += path_flow;
    }

    max_flow
}

fn bfs(
    graph: &Graph,
    source: usize,
    sink: usize,
    flow: &HashMap<(usize, usize), i32>,
    capacity: &HashMap<(usize, usize), i32>,
) -> HashMap<usize, usize> {
    let mut parent = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(source);
    while let Some(node) = queue.pop_front() {
        for neighbour in graph.edges.get(&node).unwrap() {
            let residual = get_residual_capacity(node, neighbour.node, &capacity, &flow);

            if !visited.contains(&neighbour.node) && residual > 0 {
                visited.insert(neighbour.node);
                parent.insert(neighbour.node, node);
                queue.push_back(neighbour.node);

                // Если мы достигли стока, это значит, что путь найден
                if neighbour.node == sink {
                    return parent;
                }
            }
        }
    }

    parent
}

fn get_residual_capacity(
    u: usize,
    v: usize,
    capacity: &HashMap<(usize, usize), i32>,
    flow: &HashMap<(usize, usize), i32>,
) -> i32 {
    let cap = capacity.get(&(u, v)).copied().unwrap_or(0);
    let f = flow.get(&(u, v)).copied().unwrap_or(0);
    cap - f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ford_fulkerson_max_flow_10() {
        // Граф взят из статьи: https://habr.com/ru/articles/566248/
        let mut graph = Graph::new(true);

        // A=0, B=1, C=2, D=3, E=4, F=5
        for i in 0..=5 {
            graph.add_node(i);
        }

        graph.add_edge(0, 1, 1, 7).unwrap();
        graph.add_edge(0, 2, 1, 4).unwrap();
        graph.add_edge(1, 2, 1, 4).unwrap();
        graph.add_edge(1, 4, 1, 2).unwrap();
        graph.add_edge(2, 3, 1, 4).unwrap();
        graph.add_edge(2, 4, 1, 8).unwrap();
        graph.add_edge(3, 5, 1, 12).unwrap();
        graph.add_edge(4, 3, 1, 4).unwrap();
        graph.add_edge(4, 5, 1, 5).unwrap();

        let max_flow = ford_fulkerson(&graph, 0, 5);
        assert_eq!(max_flow, 10);
    }
}
