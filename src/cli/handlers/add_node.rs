use crate::{cli::print_graph::print_graph, graph::BaseGraph};

pub fn add_node_cmd(cmd_parts: &[String], graph: &mut BaseGraph<i32>) -> Result<bool, String> {
    let Some(value) = cmd_parts.get(1) else {
        return Err("Вы должны указать значение".to_string());
    };
    let Ok(value) = value.trim().parse() else {
        return Err("Значение должно быть числом".to_string());
    };
    graph.add_node(value);
    print_graph(&graph);
    Ok(true)
}
