use crate::{
    cli::print_graph::print_graph,
    graph::Graph,
    tasks::task7::{MstPrimError, mst_prim},
};

pub fn mst_prim_cmd(cmd_parts: &[String], graph: &mut Graph) -> Result<bool, String> {
    let Some(start_node) = cmd_parts.get(1) else {
        return Err("Вы должны указать начальную вершину".to_string());
    };

    let Ok(start_node) = start_node.parse() else {
        return Err("Начальная вершина должна быть целым неотрицательным числом".to_string());
    };

    match mst_prim(graph, start_node) {
        Ok(mst) => {
            *graph = mst;
            Ok(print_graph(graph))
        }
        Err(e) => match e {
            MstPrimError::StartNodeDoesNotExist => {
                Err("Начальная вершина не существует".to_string())
            }
            MstPrimError::DirectedGraph => Err("Граф должен быть неориентированным".to_string()),
        },
    }
}
