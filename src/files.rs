use std::{fs::File, path::Path};

use crate::graph::Graph;

pub fn save_graph(graph: &Graph, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, graph)?;
    Ok(())
}

pub fn load_graph(path: impl AsRef<Path>) -> anyhow::Result<Graph> {
    let file = File::open(path)?;
    let graph = serde_json::from_reader(file)?;
    Ok(graph)
}
