use crate::{
    cli::{
        handlers::{
            add_arc::add_arc_cmd, add_node::add_node_cmd, add_rib::add_rib_cmd,
            in_degree::in_degree_cmd, node_with_greater_outdegree::node_with_greater_outdegree_cmd,
            out_degree::out_degree_cmd, remove_arc::remove_arc_cmd, remove_node::remove_node_cmd,
            remove_rib::remove_rib_cmd, save::save_cmd, sym_diff::sym_diff_cmd,
        },
        print_graph::print_graph,
    },
    graph::Graph,
};

pub fn dispatch_cmd(cmd_parts: &[String], mut graph: &mut Graph) -> Result<bool, String> {
    let Some(cmd) = cmd_parts.get(0) else {
        return Err("Вы должны указать команду".to_string());
    };

    match cmd.as_str() {
        "print" => Ok(print_graph(&graph)),
        "clean" => {
            *graph = Graph::new(graph.directed);
            Ok(print_graph(&graph))
        }
        "add_node" => add_node_cmd(cmd_parts, &mut graph),
        "remove_node" => remove_node_cmd(cmd_parts, &mut graph),
        "add_arc" => add_arc_cmd(cmd_parts, &mut graph),
        "add_rib" => add_rib_cmd(cmd_parts, &mut graph),
        "remove_arc" => remove_arc_cmd(cmd_parts, &mut graph),
        "remove_rib" => remove_rib_cmd(cmd_parts, &mut graph),
        "out_degree" => out_degree_cmd(cmd_parts, &mut graph),
        "in_degree" => in_degree_cmd(cmd_parts, &mut graph),
        "node_with_greater_outdegree" => node_with_greater_outdegree_cmd(cmd_parts, &mut graph),
        "sym_diff" => sym_diff_cmd(cmd_parts, &mut graph),
        "save" => save_cmd(cmd_parts, &mut graph),
        "exit" => {
            println!("Good luck with that!");
            return Ok(false);
        }
        "" => Err("Вы должны указать команду".to_string()),
        _ => Err("Неизвестная команда".to_string()),
    }
}
