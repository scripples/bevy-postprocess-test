use bevy::core_pipeline::core_2d::graph::Node2d;
use bevy::core_pipeline::post_process::ChromaticAberration;
use bevy::{core_pipeline::post_process::PostProcessingPlugin, prelude::*};
use bevy_postprocess_test::postprocess_plugin::PostProcessPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                transparent: false,
                // decorations: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_text, rotate))
        .run();
}

#[derive(Component)]
struct TimedText {
    time_tick: Timer,
}

#[derive(Component)]
struct SpinShape;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(1., 1., 1.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgb(1., 0., 0.),
            ..Default::default()
        })),
        SpinShape,
    ));

    commands.spawn((
        PointLight { ..default() },
        Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        Camera {
            clear_color: Color::WHITE.into(),
            ..Default::default()
        },
        ChromaticAberration {
            intensity: 0.1,
            ..Default::default()
        },
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: isize::MAX - 1,
            ..Default::default()
        },
        ChromaticAberration {
            intensity: 0.1,
            ..Default::default()
        },
    ));

    commands.spawn((
        Text2d::new(" AAAAHHHHHHHH WHYYYYYYYYYYYYYYY"),
        TextColor::BLACK,
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        TimedText {
            time_tick: Timer::from_seconds(0.10, TimerMode::Repeating),
        },
    ));
}

fn rotate(mut query: Query<&mut Transform, With<SpinShape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

fn update_text(
    time: Res<Time>,
    mut text_query: Query<(&mut Text2d, &mut Transform, &mut TimedText), With<Text2d>>,
) {
    for (mut text, mut transform, mut timed_text) in &mut text_query {
        if timed_text.time_tick.tick(time.delta()).finished() {
            let mut chars = text.0.chars().collect::<Vec<char>>();
            chars.rotate_right(1);
            text.0 = chars.iter().collect();
        }

        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs());
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
    }
}
