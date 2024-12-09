use bevy::prelude::*;
use crate::dialogue::data::*;

mod systems;

#[derive(Resource, Default)]
pub struct DialogueState {
    pub current_graph: Option<DialogueGraph>,
    pub current_node: Option<NodeId>,
}

#[derive(Event)]
pub struct DialogueProgress;

pub(crate) struct DialogueRuntimePlugin;

impl Plugin for DialogueRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DialogueState>()
            .add_event::<DialogueProgress>()
            .add_event::<StartDialogueEvent>()
            .add_event::<DialogueChoiceMadeEvent>()
            .add_systems(Update, systems::handle_dialogue_progression);
    }
}
