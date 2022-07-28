use bevy::prelude::*;
// use std::cmp::{max, min};
use super::resource::{Resource, ResourceType, ResourceTypes, STARTING_RESOURCES};

pub struct KingdomPlugin;

#[derive(Debug, Component)]
pub struct Kingdom;

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
    for i in 1..=2 {
        commands
            .spawn()
            .insert(Name::new(format!("Kingdom {}", i).to_string()))
            .with_children(|kingdom| {
                for resource_type in STARTING_RESOURCES.iter() {
                    kingdom
                        .spawn()
                        .insert(Resource {
                            value: 50,
                            change: 0,
                        })
                        .insert(ResourceType(*resource_type))
                        .insert(KingdomID(i));
                }
            })
            .insert(KingdomID(i))
            .insert(Kingdom);
    }
}
