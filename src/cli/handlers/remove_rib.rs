use crate::{
    cli::print_graph::print_graph,
    graph::{Graph, GraphRemoveRibError},
};

pub fn remove_rib_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
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

    match graph.remove_rib(first, second) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            GraphRemoveRibError::FirstNodeDoesNotExist => {
                Err("Первая вершина не существует".to_string())
            }
            GraphRemoveRibError::SecondNodeDoesNotExist => {
                Err("Вторая вершина не существует".to_string())
            }
            GraphRemoveRibError::RibDoesNotExist => {
                Err("Ребра между вершинами не существует".to_string())
            }
        },
    }
}
