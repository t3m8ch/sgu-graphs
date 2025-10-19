use crate::{
    cli::print_graph::print_graph,
    graph::{BaseGraph, DirectedGraph, RemoveArcError},
};

pub fn remove_rib_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if directed {
        return Err("Граф ориентированный, удаление рёбер не поддерживается".to_string());
    }
    let Some(first) = cmd_parts.get(1) else {
        return Err("Вы должны указать первую вершину".to_string());
    };
    let Ok(first) = first.trim().parse() else {
        return Err("Первая вершина должна быть числом".to_string());
    };
    let Some(second) = cmd_parts.get(2) else {
        return Err("Вы должны указать вторую вершину".to_string());
    };
    let Ok(second) = second.trim().parse() else {
        return Err("Вторая вершина должна быть числом".to_string());
    };
    let mut directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.remove_arc(first, second) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            RemoveArcError::FromNodeDoesNotExist => Err("Первая вершина не существует".to_string()),
            RemoveArcError::ToNodeDoesNotExist => Err("Вторая вершина не существует".to_string()),
            RemoveArcError::ArcDoesNotExist => {
                Err("Дуги между вершинами не существует".to_string())
            }
        },
    }
}
