use bevy::prelude::*;
use wireworld_xorshift_798::State;

const INITIAL_STATE: &str = include_str!("../xorshift.rle");
const WIDTH: f32 = 398.;
const HEIGHT: f32 = 206.;
const SCALE: f32 = 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH * SCALE, HEIGHT * SCALE).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .add_systems(Startup, (setup_camera, setup_map))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_map(mut commands: Commands) {
    let rle = ca_formats::rle::Rle::new(INITIAL_STATE).unwrap();
    let state = State::from(rle);

    commands
        .spawn((
            Transform::from_xyz(-WIDTH * SCALE / 2., HEIGHT * SCALE / 2., 0.),
            Visibility::default(),
        ))
        .with_children(|builder| {
            state.cells.indexed_iter().for_each(
                |((y, x), state): ((usize, usize), &u8)| match state {
                    0 => (),
                    _ => {
                        builder.spawn((
                            Sprite {
                                custom_size: Some(Vec2::splat(SCALE)),
                                color: [
                                    Color::srgba(0., 0., 0., 255.),
                                    Color::srgba(0., 0., 255., 255.),
                                    Color::srgba(255., 255., 255., 255.),
                                    Color::srgba(255., 255., 0., 255.),
                                ][*state as usize],
                                ..default()
                            },
                            Transform::from_xyz(SCALE * x as f32, -SCALE * y as f32, 0.),
                        ));
                    }
                },
            );
        });
}
