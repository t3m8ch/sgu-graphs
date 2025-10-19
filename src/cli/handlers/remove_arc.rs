use crate::{
    cli::print_graph::print_graph,
    graph::{BaseGraph, DirectedGraph, RemoveArcError},
};

pub fn remove_arc_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, удаление дуг не поддерживается".to_string());
    }
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
    let mut directed_graph: DirectedGraph<i32> = graph.into();
    match directed_graph.remove_arc(from, to) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            RemoveArcError::FromNodeDoesNotExist => {
                Err("Начальная вершина не существует".to_string())
            }
            RemoveArcError::ToNodeDoesNotExist => Err("Конечная вершина не существует".to_string()),
            RemoveArcError::ArcDoesNotExist => {
                Err("Дуги между вершинами не существует".to_string())
            }
        },
    }
}
