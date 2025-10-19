use std::io::Write;

use crate::{
    cli::{dispatch_cmd::dispatch_cmd, init_graph::init_graph},
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
