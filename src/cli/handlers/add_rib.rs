use crate::{
    cli::print_graph::print_graph,
    graph::{AddRibError, BaseGraph, UndirectedGraph},
};

pub fn add_rib_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if directed {
        return Err("Граф ориентированный, добавление рёбер не поддерживается".to_string());
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
    let mut undirected_graph: UndirectedGraph<i32> = graph.into();
    match undirected_graph.add_rib(first, second) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            AddRibError::FirstNodeDoesNotExist => Err("Первая вершина не существует".to_string()),
            AddRibError::SecondNodeDoesNotExist => Err("Вторая вершина не существует".to_string()),
            AddRibError::RibAlreadyExists => Err("Ребро уже существует".to_string()),
        },
    }
}
