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

pub const STARTING_RESOURCES: [ResourceTypes; 6] = [
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
pub struct ResourceModification(Box<dyn Fn(KingdomResources) -> KingdomResources + Send + Sync>);

impl ResourceModification {
    pub fn IncMod(resource: ResourceTypes, inc: usize) -> Self {
        return ResourceModification(Box::new(move |kingdom_res: KingdomResources| {
            let mut new_resources: KingdomResources = KingdomResources::new();
            new_resources.0.insert(resource, inc);
            return new_resources;
        }));
    }
}

#[derive(Clone)]
pub struct KingdomResources(HashMap<ResourceTypes, usize>);

impl KingdomResources {
    pub fn new() -> KingdomResources {
        let mut resources = HashMap::new();
        for resource_type in ResourceTypes::iter() {
            resources.insert(resource_type, 0);
        }
        return KingdomResources(resources);
    }

    pub fn set(&mut self, resource: ResourceTypes, value: usize) {
        (*self).0.insert(resource, value);
    }

    pub fn add(&mut self, resource: ResourceTypes, value: usize) {
        *((*self).0.entry(resource).or_insert(0)) += value;
    }
}
