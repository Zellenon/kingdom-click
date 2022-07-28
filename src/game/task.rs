use super::resource::ResourceModification;
use bevy::prelude::*;

pub struct Task {
    pub name: &'static str,
    pub description: &'static str,
    pub completion_message: &'static str,
    pub max_progress: f32,
    pub progress: f32,
    pub completion_outcome: TaskOutcome,
}

pub enum TaskOutcome {
    ResourceOutcome(ResourceModification),
}

pub struct GodStats {
    click_power: usize,
}
