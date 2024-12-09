use bevy::prelude::*;

mod components;
mod systems;

pub use components::{DialogueBox, DialoguePortrait};

pub(crate) struct DialogueUiPlugin;

impl Plugin for DialogueUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            systems::update_dialogue_display,
            systems::handle_dialogue_interaction,
        ));
    }
}
