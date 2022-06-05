use bevy::prelude::*;
use bevy::{log::LogSettings, window::PresentMode, winit::WinitSettings};

use game::GamePlugin;
use gui::GUIPlugin;
use kingdom::*;

mod game;
mod gui;
mod guiboiler;
mod kingdom;

// use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

const BACKGROUND_COLOR: Color = Color::rgb(0.05, 0.066, 0.09);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Playing,
    MainMenu,
}

// struct Handles {
//     levels: Vec<Handle<Level>>,
//     fonts: Vec<Handle<Font>>,
// }

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: String::from("Kingdom Click"),
            width: 1280.,
            height: 800.,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy".to_string()),
            ..Default::default()
        })
        .insert_resource(LogSettings {
            ..Default::default()
        });

    app.add_state(AppState::MainMenu);

    app.add_plugins(DefaultPlugins)
        .add_plugin(KingdomPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GUIPlugin);

    // app.add_system(button_system);

    app.run();
}
