//! Dialogue system for Bevy games.

mod data;
mod runtime;
#[cfg(feature = "bevy_dialogue_ui")]
mod ui;
#[cfg(feature = "bevy_dialogue_editor")]
mod editor;

pub use data::*;
pub use runtime::*;

use bevy::prelude::*;

/// Main plugin for the dialogue system
pub struct DialoguePlugin {
    /// Whether to include the editor functionality
    #[cfg(feature = "bevy_dialogue_editor")]
    pub enable_editor: bool,
    /// Whether to use the default UI implementation
    #[cfg(feature = "bevy_dialogue_ui")]
    pub use_default_ui: bool,
}

impl Default for DialoguePlugin {
    fn default() -> Self {
        Self {
            #[cfg(feature = "bevy_dialogue_editor")]
            enable_editor: false,
            #[cfg(feature = "bevy_dialogue_ui")]
            use_default_ui: true,
        }
    }
}

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        // Core runtime is always included
        app.add_plugins(runtime::DialogueRuntimePlugin);

        // Optional UI implementation
        #[cfg(feature = "bevy_dialogue_ui")]
        if self.use_default_ui {
            app.add_plugins(ui::DialogueUiPlugin);
        }

        // Optional editor
        #[cfg(feature = "bevy_dialogue_editor")]
        if self.enable_editor {
            app.add_plugins(editor::DialogueEditorPlugin);
        }
    }
}

pub mod prelude {
    pub use super::data::{DialogueGraph, DialogueNode, SpeakerInfo};
    pub use super::runtime::{DialogueState, DialogueProgress};
    
    #[cfg(feature = "bevy_dialogue_ui")]
    pub use super::ui::{DialogueBox, DialoguePortrait};
}
