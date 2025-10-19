use crate::{
    cli::print_graph::print_graph,
    graph::{Graph, GraphRemoveEdgeError},
};

pub fn remove_arc_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
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

    match graph.remove_edge(from, to) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            GraphRemoveEdgeError::FromNodeDoesNotExist => {
                Err("Начальная вершина не существует".to_string())
            }
            GraphRemoveEdgeError::ToNodeDoesNotExist => {
                Err("Конечная вершина не существует".to_string())
            }
            GraphRemoveEdgeError::UndirectedGraph => {
                Err("Граф неориентированный, удаление дуг не поддерживается".to_string())
            }
            GraphRemoveEdgeError::EdgeDoesNotExist => {
                Err("Дуги между вершинами не существует".to_string())
            }
        },
    }
}
