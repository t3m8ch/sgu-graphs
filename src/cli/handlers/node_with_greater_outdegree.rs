use crate::graph::{BaseGraph, DirectedGraph, NodesWithGreaterOutdegreeError};

pub fn node_with_greater_outdegree_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, эта команда не поддерживается".to_string());
    }
    let Some(node_id) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    let directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.nodes_with_greater_outdegree(node_id) {
        Ok(nodes) => {
            println!(
                "Вершины, у которых полустепень захода больше, чем у {} = {:?}",
                node_id, nodes
            );
            Ok(true)
        }
        Err(e) => match e {
            NodesWithGreaterOutdegreeError::NodeDoesNotExist => {
                Err("Вершина не существует".to_string())
            }
        },
    }
}
