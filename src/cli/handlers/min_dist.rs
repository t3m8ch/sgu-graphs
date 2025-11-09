use crate::{
    graph::Graph,
    tasks::task9::{MinDistanceError, min_distance},
};

pub fn min_dist_cmd(graph: &Graph) -> Result<bool, String> {
    match min_distance(graph) {
        Ok((node, distance)) => {
            println!("Вершина {node} с минимальной суммой расстояний до других вершин {distance}");
            Ok(true)
        }
        Err(e) => match e {
            MinDistanceError::NegativeCycle => {
                Err("Граф содержит цикл с отрицательным весом".to_string())
            }
            MinDistanceError::EmptyGraph => {
                Err("Невозможно запустить для пустого графа".to_string())
            }
        },
    }
}
