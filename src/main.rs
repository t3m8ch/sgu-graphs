use crate::cli::{cmd_loop::cmd_loop, init_graph::init_graph};

pub mod cli;
pub mod files;
pub mod graph;

fn main() {
    let (directed, graph) = init_graph();
    cmd_loop(graph, directed);
}
