use crate::{
    cli::print_graph::print_graph,
    files::load_graph,
    graph::{BaseGraph, DirectedGraph},
};

pub fn sym_diff_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    if !directed {
        return Err("Граф неориентированный, эта команда не поддерживается".to_string());
    }
    let Some(first_path) = cmd_parts.get(1) else {
        return Err("Вы должны указать путь для первого графа".to_string());
    };
    let Some(second_path) = cmd_parts.get(2) else {
        return Err("Вы должны указать путь для второго графа".to_string());
    };
    let Ok(mut first_graph) = load_graph(first_path) else {
        return Err(format!("Ошибка при загрузке первого графа: {}", first_path));
    };
    let Ok(mut second_graph) = load_graph(second_path) else {
        return Err(format!(
            "Ошибка при загрузке второго графа: {}",
            second_path
        ));
    };
    if !first_graph.directed {
        return Err("Первый граф должен быть ориентированным".to_string());
    }
    if !second_graph.directed {
        return Err("Второй граф должен быть ориентированным".to_string());
    }

    let first_graph: DirectedGraph<i32> = (&mut first_graph.graph).into();
    let second_graph: DirectedGraph<i32> = (&mut second_graph.graph).into();
    *graph = first_graph.symmetric_diff(&second_graph);

    Ok(print_graph(&graph))
}
