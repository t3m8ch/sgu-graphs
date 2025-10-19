use crate::cli::{cmd_loop::cmd_loop, init_graph::init_graph};

pub mod cli;
pub mod files;
pub mod graph;
pub mod tasks;

fn main() {
    cmd_loop(init_graph());
}
