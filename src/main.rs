use bevy::prelude::*;
use wireworld_xorshift_798::State;

const INITIAL_STATE: &str = include_str!("../xorshift.rle");
const WIDTH: f32 = 398.;
const HEIGHT: f32 = 206.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_map))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_map(mut commands: Commands) {
    let sprite_size = 2.;

    let rle = ca_formats::rle::Rle::new(INITIAL_STATE).unwrap();
    let state = State::from(rle);

    commands
        .spawn((
            Transform::from_xyz(
                -WIDTH * sprite_size / 2.,
                HEIGHT * sprite_size / 2.,
                0.,
            ),
            Visibility::default(),
        ))
        .with_children(|builder| {
            state.cells
                .indexed_iter()
                .for_each(|((y, x), state): ((usize, usize), &u8)| {
                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(sprite_size)),
                            color: [
                                Color::srgba(0., 0., 0., 255.),
                                Color::srgba(0., 0., 255., 255.),
                                Color::srgba(255., 255., 255., 255.),
                                Color::srgba(255., 255., 0., 255.),
                            ][*state as usize],
                            ..default()
                        },
                        Transform::from_xyz(sprite_size * x as f32, -sprite_size * y as f32, 0.),
                    ));
                });
        });
}
