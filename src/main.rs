use std::io::Write;

use crate::{
    cli::{
        handlers::{
            add_arc::add_arc_cmd, add_node::add_node_cmd, add_rib::add_rib_cmd,
            in_degree::in_degree_cmd, node_with_greater_outdegree::node_with_greater_outdegree_cmd,
            out_degree::out_degree_cmd, remove_arc::remove_arc_cmd, remove_node::remove_node_cmd,
            remove_rib::remove_rib_cmd, save::save_cmd, sym_diff::sym_diff_cmd,
        },
        init_graph::init_graph,
        print_graph::print_graph,
    },
    graph::BaseGraph,
};

pub mod cli;
pub mod files;
pub mod graph;

fn main() {
    let (directed, graph) = init_graph();
    command_loop(graph, directed);
}

fn command_loop(mut graph: BaseGraph<i32>, directed: bool) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match dispatch_cmd(&input, &mut graph, directed) {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn dispatch_cmd(
    cmd_parts: &[String],
    mut graph: &mut BaseGraph<i32>,
    directed: bool,
) -> Result<bool, String> {
    let Some(cmd) = cmd_parts.get(0) else {
        return Err("Вы должны указать команду".to_string());
    };

    match cmd.as_str() {
        "print" => Ok(print_graph(&graph)),
        "clean" => {
            *graph = BaseGraph::new();
            Ok(print_graph(&graph))
        }
        "add_node" => add_node_cmd(cmd_parts, &mut graph),
        "remove_node" => remove_node_cmd(cmd_parts, &mut graph, directed),
        "add_arc" => add_arc_cmd(cmd_parts, &mut graph, directed),
        "add_rib" => add_rib_cmd(cmd_parts, &mut graph, directed),
        "remove_arc" => remove_arc_cmd(cmd_parts, &mut graph, directed),
        "remove_rib" => remove_rib_cmd(cmd_parts, &mut graph, directed),
        "out_degree" => out_degree_cmd(cmd_parts, &mut graph, directed),
        "in_degree" => in_degree_cmd(cmd_parts, &mut graph, directed),
        "node_with_greater_outdegree" => {
            node_with_greater_outdegree_cmd(cmd_parts, &mut graph, directed)
        }
        "sym_diff" => sym_diff_cmd(cmd_parts, &mut graph, directed),
        "save" => save_cmd(cmd_parts, &mut graph, directed),
        "exit" => {
            println!("Good luck with that!");
            return Ok(false);
        }
        "" => Err("Вы должны указать команду".to_string()),
        _ => Err("Неизвестная команда".to_string()),
    }
}
