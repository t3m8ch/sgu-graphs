use crate::{graph::Graph, tasks::task11::ford_fulkerson};

pub fn ford_fulkerson_cmd(cmd_parts: &[String], graph: &Graph) -> Result<bool, String> {
    let source = match cmd_parts.get(1).map(|c| c.parse()) {
        Some(Ok(source)) => source,
        Some(Err(_)) => return Err("Исток должен быть числом".to_string()),
        None => return Err("Исток не указан".to_string()),
    };

    let sink = match cmd_parts.get(2).map(|c| c.parse()) {
        Some(Ok(sink)) => sink,
        Some(Err(_)) => return Err("Сток должен быть числом".to_string()),
        None => return Err("Сток не указан".to_string()),
    };

    println!(
        "Максимальный поток: {}",
        ford_fulkerson(graph, source, sink)
    );
    Ok(true)
}
