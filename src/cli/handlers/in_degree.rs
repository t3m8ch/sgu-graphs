use crate::{
    graph::Graph,
    tasks::task2::{IncomingNodesError, get_incoming_nodes},
};

pub fn in_degree_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    let Some(node) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };

    match get_incoming_nodes(graph, node) {
        Ok(incoming_nodes) => {
            println!(
                "Полустепень захода вершины {} = {}",
                node,
                incoming_nodes.len()
            );
            Ok(true)
        }
        Err(e) => match e {
            IncomingNodesError::NodeDoesNotExist => Err("Вершина не существует".to_string()),
            IncomingNodesError::UndirectedGraph => Err(
                "Граф неориентированный, получение полустепени захода не поддерживается"
                    .to_string(),
            ),
        },
    }
}
