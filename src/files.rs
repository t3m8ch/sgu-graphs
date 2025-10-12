use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::graph::BaseGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSave<T> {
    pub graph: BaseGraph<T>,
    pub directed: bool,
}

pub fn save_graph<T>(graph: &GraphSave<T>, path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + serde::Serialize,
{
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, graph)?;
    Ok(())
}

pub fn load_graph<T>(path: impl AsRef<Path>) -> anyhow::Result<GraphSave<T>>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let graph = serde_json::from_reader(file)?;
    Ok(graph)
}
