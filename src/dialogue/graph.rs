use bevy::prelude::*;
use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::collections::HashMap;

use super::data::{DialogueChoice, DialogueNode, DialogueNext, NodeId};

/// Internal representation of the dialogue graph using petgraph
#[derive(Debug, Default)]
pub struct DialogueGraphNavigator {
    /// The actual graph structure
    graph: DiGraph<NodeId, ChoiceInfo>,
    /// Mapping from NodeId to petgraph's NodeIndex
    node_map: HashMap<NodeId, NodeIndex>,
}

/// Information about a choice edge in the graph
#[derive(Debug, Clone)]
pub struct ChoiceInfo {
    pub choice_text: String,
}

impl DialogueGraphNavigator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: &DialogueNode) -> NodeIndex {
        let node_idx = self.graph.add_node(node.id);
        self.node_map.insert(node.id, node_idx);

        // Add edges based on the node's next field
        if let Some(next_idx) = self.node_map.get(&node.id) {
            match &node.next {
                DialogueNext::Single(Some(next_id)) => {
                    if let Some(target_idx) = self.node_map.get(next_id) {
                        self.graph.add_edge(
                            *next_idx,
                            *target_idx,
                            ChoiceInfo {
                                choice_text: String::new(),
                            },
                        );
                    }
                }
                DialogueNext::Choices(choices) => {
                    for choice in choices {
                        if let Some(target_idx) = self.node_map.get(&choice.next_node) {
                            self.graph.add_edge(
                                *next_idx,
                                *target_idx,
                                ChoiceInfo {
                                    choice_text: choice.text.clone(),
                                },
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        node_idx
    }

    /// Get all nodes that can be reached from this node
    pub fn get_next_nodes(&self, node_id: NodeId) -> Vec<NodeId> {
        if let Some(idx) = self.node_map.get(&node_id) {
            self.graph
                .edges(*idx)
                .map(|edge| *self.graph.node_weight(edge.target()).unwrap())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Check if the graph contains any cycles
    pub fn has_cycles(&self) -> bool {
        algo::is_cyclic_directed(&self.graph)
    }

    /// Find all nodes that can't be reached from the start node
    pub fn find_unreachable_nodes(&self, start_node: NodeId) -> Vec<NodeId> {
        if let Some(start_idx) = self.node_map.get(&start_node) {
            let reachable: Vec<_> = algo::kosaraju_scc(&self.graph)
                .into_iter()
                .flat_map(|scc| scc)
                .collect();

            self.graph
                .node_indices()
                .filter(|&idx| !reachable.contains(&idx))
                .map(|idx| *self.graph.node_weight(idx).unwrap())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find all leaf nodes (nodes with no outgoing edges)
    pub fn find_leaf_nodes(&self) -> Vec<NodeId> {
        self.graph
            .node_indices()
            .filter(|&idx| self.graph.edges(idx).count() == 0)
            .map(|idx| *self.graph.node_weight(idx).unwrap())
            .collect()
    }

    /// Validate the graph structure
    pub fn validate(&self, start_node: NodeId) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check for cycles
        if self.has_cycles() {
            errors.push(ValidationError::ContainsCycles);
        }

        // Check for unreachable nodes
        let unreachable = self.find_unreachable_nodes(start_node);
        if !unreachable.is_empty() {
            errors.push(ValidationError::UnreachableNodes(unreachable));
        }

        // Check for unfinished paths (leaf nodes that aren't explicitly marked as endings)
        let leaves = self.find_leaf_nodes();
        if !leaves.is_empty() {
            errors.push(ValidationError::UnfinishedPaths(leaves));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Errors that can occur in dialogue graph validation
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Dialogue graph contains cycles")]
    ContainsCycles,
    #[error("Found unreachable nodes: {0:?}")]
    UnreachableNodes(Vec<NodeId>),
    #[error("Found unfinished paths ending at: {0:?}")]
    UnfinishedPaths(Vec<NodeId>),
}