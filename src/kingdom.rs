use bevy::prelude::*;
// use std::cmp::{max, min};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

pub struct KingdomPlugin;

#[derive(Debug, Component)]
pub struct Kingdom;

#[derive(Debug, Component)]
pub struct Resource {
    pub value: usize,
}

#[derive(Component)]
pub struct ResourceType(pub ResourceTypeEnum);

impl Default for ResourceType {
    fn default() -> Self {
        Self(ResourceTypeEnum::ERROR)
    }
}

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
pub struct Name(pub String);

#[derive(Debug, Component, Default)]
pub struct KingdomID(pub usize);

#[derive(Bundle)]
struct KingdomBundle {
    name: Name,
    food: Resource,
    happpiness: Resource,
    manpower: Resource,
    military: Resource,
    faith: Resource,
}

impl Plugin for KingdomPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_kingdoms);
    }
}

fn setup_kingdoms(mut commands: Commands) {
    let starter_resources = [
        ResourceTypeEnum::Food,
        ResourceTypeEnum::Industry,
        ResourceTypeEnum::Faith,
        ResourceTypeEnum::Populace,
        ResourceTypeEnum::Military,
        ResourceTypeEnum::Happiness,
    ];
    for i in 1..=2 {
        commands
            .spawn()
            .insert(Name(format!("Kingdom {}", i).to_string()))
            .with_children(|kingdom| {
                for resource_type in starter_resources.iter() {
                    kingdom
                        .spawn()
                        .insert(Resource { value: 50 })
                        .insert(ResourceType(*resource_type))
                        .insert(KingdomID(i));
                }
            })
            .insert(KingdomID(i))
            .insert(Kingdom);
    }
}
