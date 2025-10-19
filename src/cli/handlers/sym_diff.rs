use crate::{
    cli::print_graph::print_graph,
    files::load_graph,
    graph::Graph,
    tasks::task4::{SymDiffError, sym_diff},
};

pub fn sym_diff_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    if !graph.directed {
        return Err("Граф неориентированный, эта команда не поддерживается".to_string());
    }
    let Some(first_path) = cmd_parts.get(1) else {
        return Err("Вы должны указать путь для первого графа".to_string());
    };
    let Some(second_path) = cmd_parts.get(2) else {
        return Err("Вы должны указать путь для второго графа".to_string());
    };
    let Ok(first_graph) = load_graph(first_path) else {
        return Err(format!("Ошибка при загрузке первого графа: {}", first_path));
    };
    let Ok(second_graph) = load_graph(second_path) else {
        return Err(format!(
            "Ошибка при загрузке второго графа: {}",
            second_path
        ));
    };

    match sym_diff(&first_graph, &second_graph) {
        Ok(new_graph) => {
            *graph = new_graph;
            Ok(print_graph(&graph))
        }
        Err(e) => match e {
            SymDiffError::FirstUndirectedGraph => {
                Err("Первый граф должен быть ориентированным".to_string())
            }
            SymDiffError::SecondUndirectedGraph => {
                Err("Второй граф должен быть ориентированным".to_string())
            }
        },
    }
}
