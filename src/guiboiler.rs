use bevy::prelude::*;

use crate::{
    gui::{
        self, ButtonTypeEnum, DisplayBundle, DisplayTypeEnum, FamilyBundle, ResourceDisplayText,
        ResourceReference, ResourceTextBundle, FONT_NAME, STANDARD_TEXT_STYLE,
    },
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
        color: Color::rgb(0.3, 0.2, 0.0).into(),
        // color: Color::NONE.into(),
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
                justify_content: JustifyContent::SpaceEvenly,
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

pub fn text(asset_server: &Res<AssetServer>, text: String, style: DisplayTypeEnum) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(FONT_NAME),
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
    resource_reference: ResourceReference,
) -> ResourceTextBundle {
    ResourceTextBundle {
        text: Text::with_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/iniya.otf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 1., 0.6),
            },
            Default::default(),
        ),
        resource_reference,
        ..default()
    }
}

pub fn display(
    asset_server: &Res<AssetServer>,
    style: DisplayTypeEnum,
    children: Vec<DisplayBundle>,
) -> FamilyBundle {
    let display = match style {
        DisplayTypeEnum::StandardText(text) => DisplayBundle::DisplayText(TextBundle {
            text: Text::with_section(text, STANDARD_TEXT_STYLE(asset_server), Default::default()),
            ..default()
        }),
        DisplayTypeEnum::ResourceText(resource) => {
            DisplayBundle::DisplayResource(ResourceTextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: asset_server.load(FONT_NAME),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                resource_reference: resource,
                // resource: ResourceType(resource),
                // kingdom: KingdomID(kingdom),
                ..default()
            })
        }
        DisplayTypeEnum::ResourceIcon(resource_type) => todo!(),
    };
    FamilyBundle {
        parent: display,
        children,
    }
}

pub fn frame(node: NodeBundle, children: Vec<DisplayBundle>) -> FamilyBundle {
    FamilyBundle {
        parent: DisplayBundle::FrameDisplay(node),
        children: children,
    }
}

pub fn none() -> Vec<DisplayBundle> {
    Vec::new()
}
