use bevy::prelude::*;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Debug, EnumIter, AsRefStr, Clone, Copy)]
pub enum ResourceTypeEnum {
    Food,
    Industry,
    Faith,
    Populace,
    Military,
    Happiness,
    ERROR,
}

#[derive(Debug, Component)]
pub struct Resource {
    pub value: usize,
    pub change: usize,
}

#[derive(Component)]
pub struct ResourceType(pub ResourceTypeEnum);

impl Default for ResourceType {
    fn default() -> Self {
        Self(ResourceTypeEnum::ERROR)
    }
}

#[derive(Component)]
pub struct ResourceModification {
    pub resource: Entity,
    pub interaction: fn(usize) -> usize,
}

pub fn ResourceIncMod(resource: Entity, inc: usize) -> ResourceModification {
    return ResourceModification {
        resource,
        interaction: |val: usize| val + inc,
    };
}
