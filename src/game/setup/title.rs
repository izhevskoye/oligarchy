use bevy::{prelude::*, render::camera::Camera};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn teardown(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        log::info!("remove ");
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera>>,
) {
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/SpaceGrotesk-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 70.0,
        color: Color::rgb(0.812, 0.812, 0.357),
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Oligarchy", text_style.clone(), text_alignment),
        transform: Transform::from_xyz(0.0, 150.0, 0.0),
        ..Default::default()
    });

    let font = asset_server.load("fonts/SpaceGrotesk-Light.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::rgb(0.812, 0.812, 0.357),
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(format!("v{}", VERSION), text_style.clone(), text_alignment),
        transform: Transform::from_xyz(0.0, -150.0, 0.0),
        ..Default::default()
    });

    commands.insert_resource(ClearColor(Color::rgb(
        0.584313725,
        0.231372549,
        0.270588235,
    )))
}
