use crate::{
    files::{GraphSave, save_graph},
    graph::BaseGraph,
};

pub fn save_cmd(
    cmd_parts: &[String],
    graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(path) = cmd_parts.get(1) else {
        return Err("Вы должны указать путь для сохранения графа".to_string());
    };
    if let Err(e) = save_graph(
        &GraphSave {
            directed,
            graph: graph.clone(),
        },
        path,
    ) {
        return Err(format!("Ошибка при сохранении графа: {e}"));
    }
    println!("Граф успешно сохранен в {}", path);
    Ok(true)
}
