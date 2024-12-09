use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for dialogue nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

/// Unique identifier for dialogue choices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChoiceId(pub u64);

/// A complete dialogue graph containing all nodes and their connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueGraph {
    /// All nodes in the graph
    pub nodes: HashMap<NodeId, DialogueNode>,
    /// Starting node ID
    pub start_node: NodeId,
}

/// A single node in the dialogue graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    /// Unique identifier
    pub id: NodeId,
    /// Content of this node
    pub content: DialogueContent,
    /// Next node(s) - either a single next node or multiple choice options
    pub next: DialogueNext,
    /// Position in the editor (not used at runtime)
    #[serde(default)]
    pub editor_position: Vec2,
}

/// The content displayed for this dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContent {
    /// The text to display
    pub text: String,
    /// Speaker information, if any
    pub speaker: Option<SpeakerInfo>,
}

/// Information about who is speaking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerInfo {
    /// Name of the speaker
    pub name: String,
    /// Portrait/sprite to display
    pub portrait: String,
    /// Variation of the portrait (e.g., "angry", "happy")
    pub variation: Option<String>,
}

/// What comes after this node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueNext {
    /// A single next node
    Single(Option<NodeId>),
    /// Multiple choices leading to different nodes
    Choices(Vec<DialogueChoice>),
}

/// A choice option in a dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    /// Unique identifier
    pub id: ChoiceId,
    /// Text displayed for this choice
    pub text: String,
    /// The node this choice leads to
    pub next_node: NodeId,
}

impl DialogueGraph {
    pub fn new(start_node: NodeId) -> Self {
        Self {
            nodes: HashMap::new(),
            start_node,
        }
    }

    pub fn add_node(&mut self, node: DialogueNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn get_node(&self, id: NodeId) -> Option<&DialogueNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut DialogueNode> {
        self.nodes.get_mut(&id)
    }
}

/// Resource containing the currently active dialogue
#[derive(Resource)]
pub struct ActiveDialogue {
    /// The complete dialogue graph
    pub graph: DialogueGraph,
    /// Current node being displayed
    pub current_node: NodeId,
}

// Events for the dialogue system
#[derive(Event)]
pub struct StartDialogueEvent(pub DialogueGraph);

#[derive(Event)]
pub struct DialogueChoiceMadeEvent(pub ChoiceId);
