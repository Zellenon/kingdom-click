use bevy::prelude::*;

use crate::{
    gui::{self, ButtonTypeEnum, ResourceReference, ResourceTextBundle, TextTypeEnum},
    kingdom::{KingdomID, ResourceType, ResourceTypeEnum},
};

// // // // // // // // // // // // // //
//       Layouts
// // // // // // // // // // // // // //

fn column_internal(size: Size<Val>) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            size,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}

pub fn column_perc(width: f32, height: f32) -> NodeBundle {
    let size = Size::new(
        {
            if width < 0. {
                Val::Auto
            } else {
                Val::Percent(width)
            }
        },
        {
            if height < 0. {
                Val::Auto
            } else {
                Val::Percent(height)
            }
        },
    );
    column_internal(size)
}

pub fn column_px(width: f32, height: f32) -> NodeBundle {
    let size = Size::new(
        {
            if width < 0. {
                Val::Auto
            } else {
                Val::Px(width)
            }
        },
        {
            if height < 0. {
                Val::Auto
            } else {
                Val::Px(height)
            }
        },
    );
    column_internal(size)
}

pub fn row_perc(width: f32, height: f32) -> NodeBundle {
    let size = Size::new(
        {
            if width < 0. {
                Val::Auto
            } else {
                Val::Percent(width)
            }
        },
        {
            if height < 0. {
                Val::Auto
            } else {
                Val::Percent(height)
            }
        },
    );
    row_internal(size)
}

pub fn row_px(width: f32, height: f32) -> NodeBundle {
    let size = Size::new(
        {
            if width < 0. {
                Val::Auto
            } else {
                Val::Px(width)
            }
        },
        {
            if height < 0. {
                Val::Auto
            } else {
                Val::Px(height)
            }
        },
    );
    row_internal(size)
}

fn row_internal(size: Size<Val>) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            size,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}

// // // // // // // // // // // // // //
//       Individual GUI Elements
// // // // // // // // // // // // // //

pub fn button(style: ButtonTypeEnum) -> ButtonBundle {
    match style {
        ButtonTypeEnum::MainResourceButton => ButtonBundle {
            // This font is 23 px wide at font size 40
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        ButtonTypeEnum::SettingsButton => ButtonBundle {
            // This font is 23 px wide at font size 40
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(60.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            color: Color::rgb(0.8, 0.5, 0.4).into(),
            ..default()
        },
    }
}

pub fn text(asset_server: &Res<AssetServer>, text: String, style: TextTypeEnum) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/iniya.otf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..default()
    }
}

pub fn resource_text(
    asset_server: &Res<AssetServer>,
    // kingdom: usize,
    // resource: ResourceTypeEnum,
    resource_reference: ResourceReference,
) -> ResourceTextBundle {
    ResourceTextBundle {
        text: Text::with_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/iniya.otf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        resource_reference,
        // resource: ResourceType(resource),
        // kingdom: KingdomID(kingdom),
        ..default()
    }
}
