//! Demonstrates how to use transparency in 2D.
//! Shows 3 bevy logos on top of each other, each with a different amount of transparency.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let sprite_handle = asset_server.load("branding/icon.png");

    commands.spawn(SpriteBundle::from(sprite_handle.clone()));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            texture: sprite_handle.clone(),
            // Alpha channel of the color controls transparency.
            color: Color::srgba(0.0, 0.0, 1.0, 0.7),
            ..default()
        },
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            texture: sprite_handle,
            color: Color::srgba(0.0, 1.0, 0.0, 0.3),
            ..default()
        },
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
    });
}
