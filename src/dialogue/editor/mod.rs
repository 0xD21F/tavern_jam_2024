use bevy::prelude::*;

mod systems;
mod ui;

pub(crate) struct DialogueEditorPlugin;

/// Set of systems for the dialogue editor
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorSet {
    /// Handle editor UI updates
    Interface,
    /// Handle node operations
    NodeOperations,
}

impl Plugin for DialogueEditorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, 
            (EditorSet::Interface, EditorSet::NodeOperations).chain()
        );
        
        app.add_systems(Update, (
            systems::update_editor_interface.in_set(EditorSet::Interface),
            systems::handle_node_operations.in_set(EditorSet::NodeOperations),
        ));
    }
}
