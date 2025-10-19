use crate::{
    graph::Graph,
    tasks::task6::{IsAcyclicError, is_acyclic},
};

pub fn is_acyclic_cmd(graph: &Graph) -> Result<bool, String> {
    match is_acyclic(graph) {
        Ok(true) => {
            println!("Граф ацикличен");
            Ok(true)
        }
        Ok(false) => {
            println!("Граф цикличен");
            Ok(true)
        }
        Err(e) => match e {
            IsAcyclicError::UndirectedGraph => Err("Граф неориентированный".to_string()),
        },
    }
}
