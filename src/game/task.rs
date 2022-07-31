use super::resource::ResourceModification;
use bevy::prelude::*;

#[derive(Component)]
pub struct Task {
    pub name: &'static str,
    pub description: &'static str,
    pub completion_message: &'static str,
    pub max_progress: usize,
    pub progress: usize,
    pub completion_outcome: TaskOutcome,
}

impl Task {
    pub fn new(
        name: &'static str,
        description: &'static str,
        completion_message: &'static str,
        max_progress: usize,
        completion_outcome: TaskOutcome,
    ) -> Self {
        return Task {
            name,
            description,
            completion_message,
            max_progress,
            progress: 0,
            completion_outcome,
        };
    }
}

pub enum TaskOutcome {
    ResourceOutcome(ResourceModification),
}

pub struct GodStats {
    click_power: usize,
}
