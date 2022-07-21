pub mod kingdom;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use self::kingdom::{Kingdom, KingdomID, Resource, ResourceType};

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

pub struct Log(pub Vec<(String, String)>);

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
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::WaitingForGod).with_system(start_of_turn_log_edits),
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

fn clear_change(mut resource_query: Query<&mut Resource>) {
    println!("Clearing changes.");
    for mut resource in resource_query.iter_mut() {
        resource.change = 0;
    }
}

fn tally_changes(
    mut state: ResMut<State<TurnState>>,
    mut resource_query: Query<(&mut Resource, &ResourceType)>,
    mut ev_resource_changes: EventReader<ResourceAlterationEvent>,
    mut log: ResMut<Log>,
) {
    for ResourceAlterationEvent { message, changes } in ev_resource_changes.iter() {
        let mut alteration_outcomes: Vec<String> = Vec::new();
        for (entity, change) in changes {
            let (mut resource, ResourceType(resource_type)) =
                resource_query.get_mut(*entity).unwrap();
            let change = change(resource.value) - resource.value;
            (*resource).change += change;
            alteration_outcomes.push(format!(
                "{} to {}",
                change,
                resource_type.as_ref().to_string()
            ));
        }
        log.0
            .push((message.clone().to_string(), alteration_outcomes.join(", ")));
    }
    state.set(TurnState::ApplyingChanges);
}

fn apply_changes(mut state: ResMut<State<TurnState>>, mut resource_query: Query<&mut Resource>) {
    for mut resource in resource_query.iter_mut() {
        resource.value += resource.change;
    }
    state.set(TurnState::WaitingForGod);
}

fn start_of_turn_log_edits(mut log: ResMut<Log>) {
    while log.0.iter().count() > 11 {
        // TODO: Find a way to calculate this
        log.0.remove(0);
    }
    log.0.push((
        "-------------------------------------------------".to_string(),
        "".to_string(),
    ));
}

struct KingdomStats(Vec<Resource>);

fn collect_kingdom_stats(
    kingdom_query: Query<&KingdomID, With<Kingdom>>,
    resource_query: Query<(Entity, &ResourceType, &KingdomID, &Resource)>,
) {
    let mut stats: HashMap<usize, KingdomStats> = HashMap::new();
    for KingdomID(id) in kingdom_query.iter() {
        stats.insert(*id, KingdomStats(Vec::new()));
    }
}
