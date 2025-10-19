use crate::{
    cli::print_graph::print_graph,
    graph::{Graph, GraphRemoveNodeError},
};

pub fn remove_node_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    let Some(value) = cmd_parts.get(1) else {
        return Err("Вы должны указать значение".to_string());
    };
    let Ok(value) = value.trim().parse() else {
        return Err("Значение должно быть числом".to_string());
    };

    match graph.remove_node(value) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            GraphRemoveNodeError::NodeNotFound => Err("Узел не найден".to_string()),
        },
    }
}
