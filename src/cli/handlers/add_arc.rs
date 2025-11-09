use crate::{
    cli::print_graph::print_graph,
    graph::{Graph, GraphAddEdgeError},
};

pub fn add_arc_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    let Some(from) = cmd_parts.get(1) else {
        return Err("Вы должны указать начальную вершину".to_string());
    };
    let Ok(from) = from.trim().parse() else {
        return Err("Начальная вершина должна быть числом".to_string());
    };
    let Some(to) = cmd_parts.get(2) else {
        return Err("Вы должны указать конечную вершину".to_string());
    };
    let Ok(to) = to.trim().parse() else {
        return Err("Конечная вершина должна быть числом".to_string());
    };

    let weight = match cmd_parts.get(3).map(|w| w.parse()) {
        Some(Ok(weight)) => weight,
        Some(Err(_)) => return Err("Вес должен быть числом".to_string()),
        None => 1,
    };

    match graph.add_edge(from, to, weight) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            GraphAddEdgeError::FromNodeDoesNotExist => {
                Err("Начальная вершина не существует".to_string())
            }
            GraphAddEdgeError::ToNodeDoesNotExist => {
                Err("Конечная вершина не существует".to_string())
            }
            GraphAddEdgeError::UndirectedGraph => Err("Граф неориентированный".to_string()),
            GraphAddEdgeError::EdgeAlreadyExists => Err("Дуга уже существует".to_string()),
        },
    }
}
