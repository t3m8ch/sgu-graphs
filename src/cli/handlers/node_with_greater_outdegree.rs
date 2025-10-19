use crate::{
    graph::Graph,
    tasks::task3::{NodesWithGreaterOutdegreeError, get_nodes_with_greater_outdegree},
};

pub fn node_with_greater_outdegree_cmd(
    cmd_parts: &[String],
    graph: &mut Graph,
) -> Result<bool, String> {
    let Some(node) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    match get_nodes_with_greater_outdegree(graph, node) {
        Ok(nodes) => {
            println!(
                "Вершины, у которых полустепень захода больше, чем у {} = {:?}",
                node, nodes
            );
            Ok(true)
        }
        Err(e) => match e {
            NodesWithGreaterOutdegreeError::NodeDoesNotExist => {
                Err("Вершина не существует".to_string())
            }
            NodesWithGreaterOutdegreeError::UndirectedGraph => {
                Err("Граф должен быть ориентированным".to_string())
            }
        },
    }
}
