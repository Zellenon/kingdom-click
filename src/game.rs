use bevy::prelude::*;

use crate::{gui::ResourceReference, kingdom};

pub struct ResourceAlterationEvent {
    pub message: String,
    pub changes: Vec<(Entity, fn(usize) -> usize)>,
}

pub struct GodActionEvent;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    WaitingForGod,
    CountingChanges,
    ApplyingChanges,
}

pub struct Log(Vec<String>);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Log(Vec::new()));

        app.add_state(TurnState::WaitingForGod);

        app.add_event::<ResourceAlterationEvent>()
            .add_event::<GodActionEvent>();

        app.add_system(check_for_god_action);
        app.add_system_set(
            SystemSet::on_enter(TurnState::CountingChanges)
                .label("clear")
                .with_system(clear_change),
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::CountingChanges)
                .label("count")
                .with_system(tally_changes)
                .after("clear"),
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::ApplyingChanges)
                .label("apply")
                .with_system(apply_changes),
        );
    }
}

fn check_for_god_action(
    mut ev_godaction: EventReader<GodActionEvent>,
    mut state: ResMut<State<TurnState>>,
) {
    match ev_godaction.iter().next() {
        Some(_) => state.set(TurnState::CountingChanges).unwrap(),
        None => {}
    }
}

fn clear_change(mut resource_query: Query<&mut kingdom::Resource>) {
    println!("Clearing changes.");
    for mut resource in resource_query.iter_mut() {
        resource.change = 0;
    }
}

fn tally_changes(
    mut state: ResMut<State<TurnState>>,
    mut resource_query: Query<&mut kingdom::Resource>,
    mut ev_resource_changes: EventReader<ResourceAlterationEvent>,
    mut log: ResMut<Log>,
) {
    for ResourceAlterationEvent { message, changes } in ev_resource_changes.iter() {
        println!("parsing event");
        for (entity, change) in changes {
            println!("parsing change");
            let mut resource = resource_query.get_mut(*entity).unwrap();
            let change = change(resource.value) - resource.value;
            (*resource).change += change;
        }
        log.0.push(message.clone().to_string());
    }
    state.set(TurnState::ApplyingChanges);
}

fn apply_changes(
    mut state: ResMut<State<TurnState>>,
    mut resource_query: Query<&mut kingdom::Resource>,
) {
    for mut resource in resource_query.iter_mut() {
        resource.value += resource.change;
    }
    state.set(TurnState::WaitingForGod);
    println!("Finished Turn.");
}
