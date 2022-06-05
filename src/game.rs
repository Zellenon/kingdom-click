use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    WaitingForGod,
    CountingChanges,
    ApplyingChanges,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(TurnState::WaitingForGod);
    }
}
