pub mod boilerplate;

use bevy::text::Text2dBounds;
use bevy::{prelude::*, ui::FocusPolicy};

use self::boilerplate::*;
use crate::game::kingdom::{self, Kingdom, KingdomID};
use crate::game::resource::*;
use crate::game::{GodActionEvent, Log, ResourceAlterationEvent, TurnState};
use crate::AppState;

pub const FONT_NAME: &str = "fonts/Rise of Kingdom.ttf";

pub fn STANDARD_TEXT_STYLE(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FONT_NAME),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    }
}

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct MainMenuStartButton;

#[derive(Component)]
pub struct GodActionButton;

#[derive(Component)]
pub struct ResourceInteractionButton {
    pub interactions: Vec<(Entity, fn(usize) -> usize)>,
    pub message: String,
}

#[derive(Component)]
pub struct GameScreen;

#[derive(Component)]
pub struct ResourceDisplayText;

#[derive(Component)]
pub struct LogText;

#[derive(Component)]
pub struct ButtonType(ButtonTypeEnum);

#[derive(Component)]
pub struct ResourceReference(pub Entity);

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

#[derive()]
pub enum DisplayTypeEnum {
    StandardText(String),
    ResourceText(ResourceReference),
    ResourceIcon(ResourceTypes),
}

#[derive()]
pub enum DisplayBundle {
    DisplayText(TextBundle),
    DisplayResource(ResourceTextBundle),
    IconDisplay(SpriteBundle),
    FrameDisplay(NodeBundle),
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

#[derive()]
pub struct FamilyBundle {
    pub parent: DisplayBundle,
    pub children: Vec<DisplayBundle>,
}

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(update_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(remove_main_menu));

        app.add_system_set(SystemSet::on_enter(AppState::Playing).with_system(spawn_game_screen))
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .label("godaction")
                    .with_system(send_god_action),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .before("godaction")
                    .with_system(resource_text_update)
                    .with_system(do_resource_interaction),
            )
            .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(remove_game_screen))
            .add_system_set(SystemSet::on_enter(TurnState::WaitingForGod).with_system(update_log));

        app.add_system(button_graphics_changes);
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
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
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
        let Resource {
            value: val,
            change: _,
        } = res_query.get_mut(*entity).unwrap();
        text.sections[0].value = val.to_string();
    }
}

fn do_resource_interaction(
    // mut res_query: Query<(Entity, &mut Resource, &ResourceType, &KingdomID)>,
    mut button_query: Query<(&Interaction, &ResourceInteractionButton), Changed<Interaction>>,
    mut ev_interactions: EventWriter<ResourceAlterationEvent>,
) {
    for (
        interaction,
        ResourceInteractionButton {
            interactions,
            message,
        },
    ) in button_query.iter_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                ev_interactions.send(ResourceAlterationEvent {
                    message: (*message).to_string(),
                    changes: interactions.clone(),
                });
            }
            _ => {}
        }
    }
}

fn send_god_action(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<GodActionButton>)>,
    mut ev_interactions: EventWriter<GodActionEvent>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ev_interactions.send(GodActionEvent);
            }
            _ => {}
        }
    }
}

fn update_log(mut log_display_query: Query<(&LogText, &mut Text)>, log_text: Res<Log>) {
    for (_, mut text) in log_display_query.iter_mut() {
        text.sections[0].value = (log_text
            .0
            .iter()
            .map(|x| {
                println!("{}", x.1);
                format!("{}\n{}", x.0, x.1)
            })
            .collect::<Vec<String>>())
        .join("\n");
    }
}

// // // // // // // // // // // // // //
//       Game Screen Changing Functions
// // // // // // // // // // // // // //

fn spawn_game_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kingdom_query: Query<(&KingdomID, &Name), With<Kingdom>>,
    resource_query: Query<(Entity, &ResourceType, &KingdomID)>,
) {
    let mut kingdom_iter = kingdom_query.iter();

    let kingdom_sidebar_generator =
        |parent: &mut ChildBuilder<'_, '_, '_>, id: &usize, name: &Name| {
            parent
                .spawn_bundle(column_perc(25., 100.))
                .with_children(|parent| {
                    parent.spawn_bundle(column_perc(100., 10.));
                    parent.spawn_bundle(text(
                        &asset_server,
                        format!("{}", name).to_string(),
                        DisplayTypeEnum::StandardText(format!("{}", name).to_string()),
                    ));
                    for (entity, ResourceType(resource_type), KingdomID(resource_kingdom)) in
                        resource_query.iter()
                    {
                        if resource_kingdom == id {
                            parent
                                .spawn_bundle(button(ButtonTypeEnum::MainResourceButton))
                                .insert(GodActionButton)
                                .insert(ResourceReference(entity))
                                .insert(ResourceInteractionButton {
                                    interactions: vec![(entity, |resource| resource + 1)],
                                    message: match resource_type {
                                        ResourceTypes::Food => "You bless the fields.",
                                        ResourceTypes::Industry => {
                                            "You inspire the laborers with vigor."
                                        }
                                        ResourceTypes::Faith => {
                                            "Minor miracles cultivate the people's faith."
                                        }
                                        ResourceTypes::Populace => {
                                            "Blessings of fertility bolster the populace."
                                        }
                                        ResourceTypes::Military => {
                                            "Visions of glorious crusades dance in their heads."
                                        }
                                        ResourceTypes::Happiness => {
                                            "You help an old woman find her keys."
                                        }
                                    }
                                    .to_string(),
                                })
                                .with_children(|button| {
                                    button.spawn_bundle(text(
                                        &asset_server,
                                        resource_type.as_ref().to_string(),
                                        DisplayTypeEnum::ResourceText(ResourceReference(entity)),
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
                            // });
                        }
                    }
                });
        };
    commands // Spawn columns
        .spawn_bundle(row_perc(100., -1.))
        .insert(GameScreen)
        .with_children(|parent| {
            // Kingdom 1 Sidebar
            let (KingdomID(id), name) = kingdom_iter.next().unwrap();
            kingdom_sidebar_generator(parent, &id, &name);

            // Log
            parent
                .spawn_bundle(column_perc(50., 100.))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Your journey begins.",
                                TextStyle {
                                    font: asset_server.load(FONT_NAME),
                                    font_size: 30.0,
                                    color: Color::rgb(0.7, 0.7, 0.7),
                                },
                                Default::default(),
                            ),
                            ..default()
                        })
                        .insert(LogText);
                });

            // Kingdom 2 Sidebar
            let (KingdomID(id), name) = kingdom_iter.next().unwrap();
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
                        DisplayTypeEnum::StandardText("Start".to_string()),
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

// pub fn spawn_with_children(master: &ChildBuilder<'_, '_, '_>, fam_bundle: FamilyBundle) {
//     let FamilyBundle {
//         parent: parent,
//         children: children,
//     } = fam_bundle;
//     let parent_bundle = match parent {
//         DisplayBundle::DisplayText(bundle) => {
//             (*master).spawn_bundle(bundle).with_children(|parent| {
//                 for child in children {
//                     match child {
//                         DisplayBundle::DisplayText(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::DisplayResource(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::IconDisplay(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::FrameDisplay(bundle) => parent.spawn_bundle(bundle),
//                     };
//                 }
//             })
//         }
//         DisplayBundle::DisplayResource(bundle) => {
//             master.spawn_bundle(bundle).with_children(|parent| {
//                 for child in children {
//                     match child {
//                         DisplayBundle::DisplayText(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::DisplayResource(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::IconDisplay(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::FrameDisplay(bundle) => parent.spawn_bundle(bundle),
//                     };
//                 }
//             })
//         }
//         DisplayBundle::IconDisplay(bundle) => master.spawn_bundle(bundle).with_children(|parent| {
//             for child in children {
//                 match child {
//                     DisplayBundle::DisplayText(bundle) => parent.spawn_bundle(bundle),
//                     DisplayBundle::DisplayResource(bundle) => parent.spawn_bundle(bundle),
//                     DisplayBundle::IconDisplay(bundle) => parent.spawn_bundle(bundle),
//                     DisplayBundle::FrameDisplay(bundle) => parent.spawn_bundle(bundle),
//                 };
//             }
//         }),
//         DisplayBundle::FrameDisplay(bundle) => {
//             master.spawn_bundle(bundle).with_children(|parent| {
//                 for child in children {
//                     match child {
//                         DisplayBundle::DisplayText(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::DisplayResource(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::IconDisplay(bundle) => parent.spawn_bundle(bundle),
//                         DisplayBundle::FrameDisplay(bundle) => parent.spawn_bundle(bundle),
//                     };
//                 }
//             })
//         }
//     };
// }
