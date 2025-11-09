use std::collections::{HashMap, HashSet};

use bon::Builder;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Сюда в будущем будут добавляться свойства рёбер/дуг
#[derive(Derivative, Serialize, Deserialize, Builder)]
#[derivative(Clone, Debug, PartialEq, Eq, Hash)]
#[builder(start_fn = value)]
pub struct Edge {
    #[builder(start_fn)]
    pub node: usize,

    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    #[builder(default = 1)]
    #[serde(default = "default_weight")]
    pub weight: i32,
}

fn default_weight() -> i32 {
    1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    pub edges: HashMap<usize, HashSet<Edge>>,
    pub directed: bool,
}

#[derive(Clone, Debug, Error)]
pub enum GraphRemoveNodeError {
    #[error("Node not found")]
    NodeNotFound,
}

#[derive(Clone, Debug, Error)]
pub enum GraphAddEdgeError {
    #[error("From node does not exist")]
    FromNodeDoesNotExist,

    #[error("To node does not exist")]
    ToNodeDoesNotExist,

    #[error("Graph is undirected")]
    UndirectedGraph,

    #[error("Edge already exists")]
    EdgeAlreadyExists,
}

#[derive(Clone, Debug, Error)]
pub enum GraphRemoveEdgeError {
    #[error("From node does not exist")]
    FromNodeDoesNotExist,

    #[error("To node does not exist")]
    ToNodeDoesNotExist,

    #[error("Graph is undirected")]
    UndirectedGraph,

    #[error("Edge does not exist")]
    EdgeDoesNotExist,
}

#[derive(Clone, Debug, Error)]
pub enum GraphAddRibError {
    #[error("From node does not exist")]
    FirstNodeDoesNotExist,

    #[error("Second node does not exist")]
    SecondNodeDoesNotExist,

    #[error("Rib already exists")]
    RibAlreadyExists,
}

#[derive(Clone, Debug, Error)]
pub enum GraphRemoveRibError {
    #[error("First node does not exist")]
    FirstNodeDoesNotExist,

    #[error("Second node does not exist")]
    SecondNodeDoesNotExist,

    #[error("Rib does not exist")]
    RibDoesNotExist,
}

// TODO: Добавить total_weight
impl Graph {
    pub fn new(directed: bool) -> Self {
        Graph {
            edges: HashMap::new(),
            directed,
        }
    }

    pub fn add_node(&mut self, value: usize) {
        self.edges.entry(value).or_insert_with(HashSet::new);
    }

    pub fn remove_node(&mut self, value: usize) -> Result<(), GraphRemoveNodeError> {
        if !self.contains_node(value) {
            return Err(GraphRemoveNodeError::NodeNotFound);
        }

        self.edges.remove(&value);
        for neighbours in self.edges.values_mut() {
            neighbours.retain(|e| e.node != value);
        }

        Ok(())
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        weight: i32,
    ) -> Result<(), GraphAddEdgeError> {
        if !self.directed {
            return Err(GraphAddEdgeError::UndirectedGraph);
        }

        if !self.contains_node(from) {
            return Err(GraphAddEdgeError::FromNodeDoesNotExist);
        }

        if !self.contains_node(to) {
            return Err(GraphAddEdgeError::ToNodeDoesNotExist);
        }

        if self.contains_edge(from, to) {
            return Err(GraphAddEdgeError::EdgeAlreadyExists);
        }

        self.edges
            .entry(from)
            .or_default()
            .insert(Edge::value(to).weight(weight).build());

        Ok(())
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), GraphRemoveEdgeError> {
        if !self.contains_node(from) {
            return Err(GraphRemoveEdgeError::FromNodeDoesNotExist);
        }

        if !self.contains_node(to) {
            return Err(GraphRemoveEdgeError::ToNodeDoesNotExist);
        }

        if !self.contains_edge(from, to) {
            return Err(GraphRemoveEdgeError::EdgeDoesNotExist);
        }

        self.edges.entry(from).or_default().retain(|e| e.node != to);

        Ok(())
    }

    pub fn add_rib(
        &mut self,
        first: usize,
        second: usize,
        weight: i32,
    ) -> Result<(), GraphAddRibError> {
        if !self.contains_node(first) {
            return Err(GraphAddRibError::FirstNodeDoesNotExist);
        }

        if !self.contains_node(second) {
            return Err(GraphAddRibError::SecondNodeDoesNotExist);
        }

        if self.contains_edge(first, second) || self.contains_edge(second, first) {
            return Err(GraphAddRibError::RibAlreadyExists);
        }

        self.edges
            .entry(first)
            .or_default()
            .insert(Edge::value(second).weight(weight).build());

        self.edges
            .entry(second)
            .or_default()
            .insert(Edge::value(first).weight(weight).build());

        Ok(())
    }

    pub fn remove_rib(&mut self, first: usize, second: usize) -> Result<(), GraphRemoveRibError> {
        if !self.contains_node(first) {
            return Err(GraphRemoveRibError::FirstNodeDoesNotExist);
        }

        if !self.contains_node(second) {
            return Err(GraphRemoveRibError::SecondNodeDoesNotExist);
        }

        if !self.contains_edge(first, second) || !self.contains_edge(second, first) {
            return Err(GraphRemoveRibError::RibDoesNotExist);
        }

        self.edges
            .entry(first)
            .or_default()
            .retain(|e| e.node != second);

        self.edges
            .entry(second)
            .or_default()
            .retain(|e| e.node != first);

        Ok(())
    }

    pub fn contains_node(&self, node: usize) -> bool {
        self.edges.contains_key(&node)
    }

    pub fn contains_edge(&self, from: usize, to: usize) -> bool {
        self.edges[&from].contains(&Edge::value(to).build())
    }
}
