use bevy::{prelude::*, ui::FocusPolicy};

use crate::guiboiler::*;
use crate::kingdom::{self, Kingdom, KingdomID, Resource, ResourceType};
use crate::AppState;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct MainMenuStartButton;

#[derive(Component)]
pub struct GodActionButton;

#[derive(Component)]
pub struct ResourceInteractionButton(
    pub fn(Entity, &mut kingdom::Resource, &ResourceType, &KingdomID) -> (),
);

#[derive(Component)]
pub struct GameScreen;

#[derive(Component)]
pub struct ResourceDisplayText;

#[derive(Component)]
pub struct ButtonType(ButtonTypeEnum);

#[derive(Component)]
pub struct ResourceReference(Entity);

impl Default for ResourceReference {
    fn default() -> Self {
        // TODO: FIX THIS. DEFAULT ENTITIES ARE JUST BAD.
        Self(Entity::from_raw(u32::MAX))
    }
}

#[derive(Hash, PartialEq)]
pub enum ButtonTypeEnum {
    MainResourceButton,
    SettingsButton,
}

#[derive(Hash)]
pub enum TextTypeEnum {
    ResourceButtonText,
}

#[derive(Bundle, Default)]
pub struct ResourceTextBundle {
    // pub resource: ResourceType,
    // pub kingdom: KingdomID,
    pub resource_reference: ResourceReference,
    pub node: Node,
    pub style: Style,
    pub text: Text,
    pub calculated_size: CalculatedSize,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(remove_main_menu));

        app.add_system_set(SystemSet::on_enter(AppState::Playing).with_system(spawn_game_screen))
            // .add_system_set(SystemSet::on_update(AppState::Playing).with_system(update_game_screen))
            .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(remove_game_screen));

        app.add_system(button_graphics_changes)
            .add_system(resource_text_update)
            .add_system(do_resource_interaction);
    }
}

fn ui_setup(mut commands: Commands) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
}

// TODO: Make a catchall function that handles cosmetic changes on buttonpress so other functions
// can just handle behavior and data.
//
// TODO: Make color constants

// // // // // // // // // // // // // //
//       Global Update Systems
// // // // // // // // // // // // // //

fn button_graphics_changes(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Pressed".to_string();
                *color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = Color::rgb(0.5, 0.5, 0.5).into();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

fn resource_text_update(
    mut text_query: Query<(&mut Text, &ResourceReference), With<ResourceDisplayText>>,
    mut res_query: Query<&Resource>,
) {
    for (mut text, ResourceReference(entity)) in text_query.iter_mut() {
        let Resource { value: val } = res_query.get_mut(*entity).unwrap();
        text.sections[0].value = val.to_string();
    }
}

fn do_resource_interaction(
    mut res_query: Query<(Entity, &mut Resource, &ResourceType, &KingdomID)>,
    mut button_query: Query<
        (&Interaction, &ResourceReference, &ResourceInteractionButton),
        (Changed<Interaction>, With<ResourceInteractionButton>),
    >,
) {
    for (interaction, ResourceReference(resource_entity), ResourceInteractionButton(command)) in
        button_query.iter_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                let (_, mut resource, resource_type, kingdom_id) =
                    res_query.get_mut(*resource_entity).unwrap();
                command(*resource_entity, &mut *resource, resource_type, kingdom_id);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

// // // // // // // // // // // // // //
//       Game Screen Changing Functions
// // // // // // // // // // // // // //

fn spawn_game_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kingdom_query: Query<(&KingdomID, &kingdom::Name), With<Kingdom>>,
    resource_query: Query<(Entity, &ResourceType, &KingdomID)>,
) {
    let mut kingdom_iter = kingdom_query.iter();

    let kingdom_sidebar_generator =
        |parent: &mut ChildBuilder<'_, '_, '_>, id: &usize, name: &String| {
            parent
                .spawn_bundle(column_perc(30., -1.))
                .with_children(|parent| {
                    parent.spawn_bundle(text(
                        &asset_server,
                        format!("Kingdom {}", id).to_string(),
                        TextTypeEnum::ResourceButtonText,
                    ));
                    // for resource in ResourceTypeEnum::iter() {
                    for (entity, ResourceType(resource_type), KingdomID(resource_kingdom)) in
                        resource_query.iter()
                    {
                        if resource_kingdom == id {
                            parent
                                .spawn_bundle(button(ButtonTypeEnum::MainResourceButton))
                                .insert(GodActionButton)
                                .insert(ResourceReference(entity))
                                .insert(ResourceInteractionButton(|_, resource, _, _| {
                                    (*resource).value += 1;
                                }))
                                .with_children(|button| {
                                    // Resource Name
                                    button.spawn_bundle(text(
                                        &asset_server,
                                        resource_type.as_ref().to_string(),
                                        TextTypeEnum::ResourceButtonText,
                                    ));

                                    // Resource Display
                                    button
                                        .spawn_bundle(resource_text(
                                            &asset_server,
                                            ResourceReference(entity), // *id,
                                                                       // *resource_type,
                                        ))
                                        .insert(ResourceDisplayText);
                                });
                        }
                    }
                });
        };
    commands
        .spawn_bundle(row_perc(100., -1.))
        .insert(GameScreen)
        .with_children(|parent| {
            // Kingdom 1 Sidebar
            let (KingdomID(id), kingdom::Name(name)) = kingdom_iter.next().unwrap();
            kingdom_sidebar_generator(parent, &id, &name);

            // Spacer
            parent.spawn_bundle(column_perc(40., -1.));

            // Kingdom 2 Sidebar
            let (KingdomID(id), kingdom::Name(name)) = kingdom_iter.next().unwrap();
            kingdom_sidebar_generator(parent, &id, &name);
        });
}

fn remove_game_screen(mut commands: Commands, menu_query: Query<Entity, With<GameScreen>>) {
    for menu in menu_query.iter() {
        commands.entity(menu.into()).despawn_recursive();
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Main Menu");
    commands
        .spawn_bundle(row_perc(100., -1.))
        .insert(MainMenuScreen)
        .with_children(|menu| {
            menu.spawn_bundle(button(ButtonTypeEnum::SettingsButton))
                .insert(ButtonType(ButtonTypeEnum::SettingsButton))
                .insert(MainMenuStartButton)
                .with_children(|button| {
                    button.spawn_bundle(text(
                        &asset_server,
                        "Start".to_string(),
                        TextTypeEnum::ResourceButtonText,
                    ));
                });
        });
}

fn update_main_menu(
    mut state: ResMut<State<AppState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuStartButton>)>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Playing).unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn remove_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuScreen>>) {
    println!("Removing Main Menu");
    for menu in menu_query.iter() {
        commands.entity(menu.into()).despawn_recursive();
    }
}
