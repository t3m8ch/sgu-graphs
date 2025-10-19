use crate::{
    graph::Graph,
    tasks::task2::{OutgoingNodesError, get_outgoing_nodes},
};

pub fn out_degree_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    let Some(node) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };

    match get_outgoing_nodes(graph, node) {
        Ok(nodes) => {
            println!("Полустепень исхода вершины {} = {}", node, nodes.len());
            Ok(true)
        }
        Err(e) => match e {
            OutgoingNodesError::NodeDoesNotExist => Err("Вершина не существует".to_string()),
            OutgoingNodesError::UndirectedGraph => Err(
                "Граф неориентированный, получение полустепени исхода не поддерживается"
                    .to_string(),
            ),
        },
    }
}
