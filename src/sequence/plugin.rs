use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo},
};

use super::*;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_sequence)
        .add_systems(Update, screen_location)
        ;
    }
}

#[derive(Component)]
struct SequenceViewSpawned;

#[derive(Component)]
struct RectangleSpawned;

fn screen_location(
    query: Query<(Entity, &SequenceView, &Text, &TextLayoutInfo), (With<SequenceViewSpawned>, Without<RectangleSpawned>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Get world coordinates of the text location, then of the third letter

    for (e, sequence, text, text_layout) in query.iter() {

        // println!("Layout: {:?}", text_layout);

        if text_layout.glyphs.len() < 3 {
            return;
        }

        let mut ec = commands.entity(e);
        ec.insert(RectangleSpawned);

        let third = &text_layout.glyphs[2];

        println!("Text Layout: {:?}", text_layout.logical_size);
        println!("Third letter: {:?}", third);

        // Given the position and size, draw a box around the third letter
        let third_x = third.position.x;
        let third_y = third.position.y;
        let third_width = third.size.x;
        let third_height = third.size.y;

        // Shows up right above it, so to fix it
        let third_y = third_y - text_layout.logical_size.y;
        let third_center = Vec3::new(third_x, third_y, 0.0);

        // draw box
        let shape = Mesh2dHandle(meshes.add(Rectangle::new(third_width, third_height)));

        let color = Color::hsla(0.0, 0.0, 0.5, 0.5);

        // Make a child of the text
        let children_commands = ec.with_children(
            |parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: shape,
                    material: materials.add(color),
                    transform: Transform::from_translation(third_center),
                    ..Default::default()
                });
            }
        );
        

        
    }


}


fn draw_sequence(
    query: Query<(Entity, &SequenceView), Without<SequenceViewSpawned>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server
        .load("fonts/monaspace-v1.101/fonts/variable/MonaspaceNeonVarVF[wght,wdth,slnt].ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text_justification = JustifyText::Left;

    for (e, sequence) in query.iter() {
        let mut ec = commands.entity(e);

        ec.insert(
            (Text2dBundle {
                text: Text::from_section(&sequence.sequence, text_style.clone())
                    .with_justify(text_justification),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                text_anchor: Anchor::TopLeft,
                ..default()
            },
            SequenceViewSpawned,
        ),
        );
    }
}
