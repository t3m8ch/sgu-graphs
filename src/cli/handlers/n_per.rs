use crate::{graph::Graph, tasks::task10::n_periphery};

pub fn n_per_cmd(cmd_parts: &[String], graph: &Graph) -> Result<bool, String> {
    let n = match cmd_parts.get(1).map(|n| n.parse::<i32>()) {
        Some(Ok(n)) => n,
        Some(Err(_)) => return Err("Первый аргумент должен быть числом".to_string()),
        None => return Err("Не передан n".to_string()),
    };

    let target = match cmd_parts.get(2).map(|t| t.parse::<usize>()) {
        Some(Ok(n)) => n,
        Some(Err(_)) => return Err("Второй аргумент должен быть числом".to_string()),
        None => return Err("Не передана вершина".to_string()),
    };

    println!("{:?}", n_periphery(graph, target, n));
    Ok(true)
}
