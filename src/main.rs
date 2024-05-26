use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};

mod sequence;

use sequence::{SequencePlugin, SequenceView};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SequencePlugin)
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SequenceView {
        sequence: "Hello, Bevy!".to_string(),
    });
}
