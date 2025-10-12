use std::collections::{HashMap, HashSet};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Сюда в будущем будут добавляться свойства рёбер/дуг
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub node_id: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseGraph<T> {
    next_id: usize,
    nodes: HashMap<usize, T>,
    edges: HashMap<usize, HashSet<Edge>>,
}

impl<T> BaseGraph<T> {
    pub fn new() -> Self {
        BaseGraph {
            next_id: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, value);
        self.edges.insert(id, HashSet::new());
        id
    }

    fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.remove(&id);
        for edge in self.edges.values_mut() {
            edge.retain(|e| e.node_id != id);
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(Edge { node_id: to });
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.remove(&Edge { node_id: to });
        }
    }

    fn has_node(&self, id: usize) -> bool {
        self.nodes.contains_key(&id)
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        if let Some(edges) = self.edges.get(&from) {
            edges.contains(&Edge { node_id: to })
        } else {
            false
        }
    }

    fn from_edges(&self, id: usize) -> HashSet<usize> {
        self.edges
            .get(&id)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|to| to.node_id)
            .collect()
    }

    fn to_edges(&self, id: usize) -> HashSet<usize> {
        self.edges
            .iter()
            .filter_map(|(from_node, to_nodes)| {
                if to_nodes.contains(&Edge { node_id: id }) {
                    Some(*from_node)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, Error)]
pub enum RemoveDirectedGraphNodeError {
    #[error("Node does not exist")]
    NodeDoesNotExist,

    #[error("Node has arcs")]
    NodeHasArcs {
        from: HashSet<usize>,
        to: HashSet<usize>,
    },
}

#[derive(Clone, Debug, Error)]
pub enum AddArcError {
    #[error("From node does not exist")]
    FromNodeDoesNotExist,

    #[error("To node does not exist")]
    ToNodeDoesNotExist,

    #[error("Arc already exists")]
    ArcAlreadyExists,
}

#[derive(Clone, Debug, Error)]
pub enum RemoveArcError {
    #[error("From node does not exist")]
    FromNodeDoesNotExist,

    #[error("To node does not exist")]
    ToNodeDoesNotExist,

    #[error("Arc does not exist")]
    ArcDoesNotExist,
}

pub struct DirectedGraph<'a, T> {
    base_graph: &'a mut BaseGraph<T>,
}

impl<'a, T> Into<DirectedGraph<'a, T>> for &'a mut BaseGraph<T> {
    fn into(self) -> DirectedGraph<'a, T> {
        DirectedGraph { base_graph: self }
    }
}

impl<'a, T> DirectedGraph<'a, T> {
    pub fn remove_node(&mut self, id: usize) -> Result<(), RemoveDirectedGraphNodeError> {
        if !self.base_graph.has_node(id) {
            Err(RemoveDirectedGraphNodeError::NodeDoesNotExist)
        } else {
            let from_edges = self.base_graph.from_edges(id);
            let to_edges = self.base_graph.to_edges(id);

            if !from_edges.is_empty() || !to_edges.is_empty() {
                Err(RemoveDirectedGraphNodeError::NodeHasArcs {
                    from: from_edges,
                    to: to_edges,
                })
            } else {
                self.base_graph.remove_node(id);
                Ok(())
            }
        }
    }

    pub fn add_arc(&mut self, from: usize, to: usize) -> Result<(), AddArcError> {
        if !self.base_graph.has_node(from) {
            Err(AddArcError::FromNodeDoesNotExist)
        } else if !self.base_graph.has_node(to) {
            Err(AddArcError::ToNodeDoesNotExist)
        } else if self.base_graph.has_edge(from, to) {
            Err(AddArcError::ArcAlreadyExists)
        } else {
            self.base_graph.add_edge(from, to);
            Ok(())
        }
    }

    pub fn remove_arc(&mut self, from: usize, to: usize) -> Result<(), RemoveArcError> {
        if !self.base_graph.has_node(from) {
            Err(RemoveArcError::FromNodeDoesNotExist)
        } else if !self.base_graph.has_node(to) {
            Err(RemoveArcError::ToNodeDoesNotExist)
        } else if !self.base_graph.has_edge(from, to) {
            Err(RemoveArcError::ArcDoesNotExist)
        } else {
            self.base_graph.remove_edge(from, to);
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum RemoveUndirectedGraphNodeError {
    #[error("Node does not exist")]
    NodeDoesNotExist,

    #[error("Node has ribs")]
    NodeHasRibs(HashSet<usize>),
}

#[derive(Debug, Clone, Error)]
pub enum AddRibError {
    #[error("First node does not exist")]
    FirstNodeDoesNotExist,

    #[error("Second node does not exist")]
    SecondNodeDoesNotExist,

    #[error("Rib already exists")]
    RibAlreadyExists,
}

#[derive(Debug, Clone, Error)]
pub enum RemoveRibError {
    #[error("First node does not exist")]
    FirstNodeDoesNotExist,

    #[error("Second node does not exist")]
    SecondNodeDoesNotExist,

    #[error("Rib does not exist")]
    RibDoesNotExist,
}

pub struct UndirectedGraph<'a, T> {
    base_graph: &'a mut BaseGraph<T>,
}

impl<'a, T> Into<UndirectedGraph<'a, T>> for &'a mut BaseGraph<T> {
    fn into(self) -> UndirectedGraph<'a, T> {
        UndirectedGraph { base_graph: self }
    }
}

impl<'a, T> UndirectedGraph<'a, T> {
    pub fn remove_node(&mut self, id: usize) -> Result<(), RemoveUndirectedGraphNodeError> {
        if !self.base_graph.has_node(id) {
            Err(RemoveUndirectedGraphNodeError::NodeDoesNotExist)
        } else {
            let node_ribs = self.base_graph.from_edges(id);
            if !node_ribs.is_empty() {
                Err(RemoveUndirectedGraphNodeError::NodeHasRibs(node_ribs))
            } else {
                self.base_graph.remove_node(id);
                Ok(())
            }
        }
    }

    pub fn add_rib(&mut self, first: usize, second: usize) -> Result<(), AddRibError> {
        if !self.base_graph.has_node(first) {
            Err(AddRibError::FirstNodeDoesNotExist)
        } else if !self.base_graph.has_node(second) {
            Err(AddRibError::SecondNodeDoesNotExist)
        } else if self.base_graph.has_edge(first, second) {
            Err(AddRibError::RibAlreadyExists)
        } else {
            self.base_graph.add_edge(first, second);
            self.base_graph.add_edge(second, first);
            Ok(())
        }
    }

    pub fn remove_rib(&mut self, first: usize, second: usize) -> Result<(), RemoveRibError> {
        if !self.base_graph.has_node(first) {
            Err(RemoveRibError::FirstNodeDoesNotExist)
        } else if !self.base_graph.has_node(second) {
            Err(RemoveRibError::SecondNodeDoesNotExist)
        } else if !self.base_graph.has_edge(first, second) {
            Err(RemoveRibError::RibDoesNotExist)
        } else {
            self.base_graph.remove_edge(first, second);
            self.base_graph.remove_edge(second, first);
            Ok(())
        }
    }
}
