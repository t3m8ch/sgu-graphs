use crate::{
    cli::print_graph::print_graph,
    graph::{Graph, GraphAddRibError},
};

pub fn add_rib_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
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
    let Ok(weight) = cmd_parts.get(3).unwrap_or(&"1".to_string()).trim().parse() else {
        return Err("Вес ребра должен быть целым числом".to_string());
    };
    match graph.add_rib(first, second, weight) {
        Ok(_) => Ok(print_graph(&graph)),
        Err(e) => match e {
            GraphAddRibError::FirstNodeDoesNotExist => {
                Err("Первая вершина не существует".to_string())
            }
            GraphAddRibError::SecondNodeDoesNotExist => {
                Err("Вторая вершина не существует".to_string())
            }
            GraphAddRibError::RibAlreadyExists => Err("Ребро уже существует".to_string()),
        },
    }
}
