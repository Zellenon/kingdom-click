use std::sync::Arc;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumCount, EnumIter};

#[derive(Debug, EnumIter, EnumCount, AsRefStr, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceTypes {
    Food,
    Industry,
    Faith,
    Populace,
    Military,
    Happiness,
}

pub const StartingResources: [ResourceTypes; 6] = [
    ResourceTypes::Food,
    ResourceTypes::Industry,
    ResourceTypes::Faith,
    ResourceTypes::Populace,
    ResourceTypes::Military,
    ResourceTypes::Happiness,
];

#[derive(Debug, Component, Clone)]
pub struct Resource {
    pub value: usize,
    pub change: usize,
}

#[derive(Component, PartialEq)]
pub struct ResourceType(pub ResourceTypes);

#[derive(Component)]
pub struct ResourceModification(
    Arc<dyn Fn(KingdomResources) -> KingdomResources + Send + Sync + 'static>,
);

#[derive(Clone)]
pub struct KingdomResources(HashMap<ResourceTypes, usize>);

impl KingdomResources {
    pub fn new() -> KingdomResources {
        let resources = HashMap::new();
        for resource_type in ResourceTypes::iter() {
            resources.insert(resource_type, 0);
        }
        return KingdomResources(resources);
    }
}

pub fn ResourceIncMod(resource: ResourceTypes, inc: usize) -> ResourceModification {
    return ResourceModification(Arc::new(|kingdom_res: KingdomResources| {
        let new_resources: KingdomResources = KingdomResources::new();
        new_resources.0.insert(resource, inc);
        return new_resources;
    }));
}
