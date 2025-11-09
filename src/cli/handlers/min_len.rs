use crate::{
    graph::Graph,
    tasks::task8::{MinimalLengthToNodesError, minimal_length_to_nodes},
};

pub fn min_len_cmd(cmd_parts: &[String], graph: &Graph) -> Result<bool, String> {
    let Some(source_node) = cmd_parts.get(1) else {
        return Err("Вы должны указать начальную вершину".to_string());
    };

    let Ok(source_node) = source_node.parse() else {
        return Err("Начальная вершина должна быть числом".to_string());
    };

    match minimal_length_to_nodes(graph, source_node) {
        Ok(result) => {
            println!("{:#?}", result);
            Ok(true)
        }
        Err(e) => match e {
            MinimalLengthToNodesError::NegativeWeight { from, to, weight } => Err(format!(
                "Вес ребра ({from}, {to}) имеет отрицательный вес {weight}"
            )),
        },
    }
}
