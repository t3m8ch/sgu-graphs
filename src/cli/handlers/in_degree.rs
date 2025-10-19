use crate::graph::{BaseGraph, DirectedGraph, InDegreeError};

pub fn in_degree_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err(
            "Граф неориентированный, получение полустепени захода не поддерживается".to_string(),
        );
    }
    let Some(node_id) = cmd_parts.get(1).and_then(|s| s.parse().ok()) else {
        return Err("Вы должны указать вершину".to_string());
    };
    let directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.in_degree(node_id) {
        Ok(degree) => {
            println!("Полустепень захода вершины {} = {}", node_id, degree);
            Ok(true)
        }
        Err(e) => match e {
            InDegreeError::NodeDoesNotExist => Err("Вершина не существует".to_string()),
        },
    }
}
