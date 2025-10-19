use crate::{
    cli::{ask_bool::ask_bool, print_graph::print_graph},
    graph::{
        BaseGraph, DirectedGraph, RemoveDirectedGraphNodeError, RemoveUndirectedGraphNodeError,
        UndirectedGraph,
    },
};

pub fn remove_node_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(value) = cmd_parts.get(1) else {
        return Err("Вы должны указать значение".to_string());
    };
    let Ok(value) = value.trim().parse() else {
        return Err("Значение должно быть числом".to_string());
    };

    if directed {
        let mut directed_graph: DirectedGraph<i32> = graph.into();
        match directed_graph.remove_node(value) {
            Ok(_) => Ok(print_graph(&graph)),
            Err(e) => match e {
                RemoveDirectedGraphNodeError::NodeDoesNotExist => Err("Узел не найден".to_string()),
                RemoveDirectedGraphNodeError::NodeHasArcs { from, to } => {
                    eprintln!(
                        "Узел имеет входящие дуги с узлами {:?}, а также исходящие дуги с узлами {:?}",
                        to, from
                    );

                    if ask_bool("Удалить все дуги") {
                        for &node in &to {
                            directed_graph.remove_arc(node, value).unwrap();
                        }
                        for &node in &from {
                            directed_graph.remove_arc(value, node).unwrap();
                        }
                        directed_graph.remove_node(value).unwrap();
                        print_graph(&graph);
                    }

                    Ok(true)
                }
            },
        }
    } else {
        let mut undirected_graph: UndirectedGraph<i32> = graph.into();
        match undirected_graph.remove_node(value) {
            Ok(_) => Ok(print_graph(&graph)),
            Err(e) => match e {
                RemoveUndirectedGraphNodeError::NodeDoesNotExist => {
                    Err("Узел не найден".to_string())
                }
                RemoveUndirectedGraphNodeError::NodeHasRibs(nodes) => {
                    eprintln!("Узел имеет рёбра с узлами {:?}", nodes);

                    if ask_bool("Удалить рёбра") {
                        for node in nodes {
                            undirected_graph.remove_rib(value, node).unwrap();
                        }
                        undirected_graph.remove_node(value).unwrap();
                        print_graph(&graph);
                    }

                    Ok(true)
                }
            },
        }
    }
}
