use bevy::{color::palettes::css::*, prelude::*};
use bevy_life::{MooreCell2d, SimulationBatch, WireWorld2dPlugin, WireWorldCellState};
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
        .add_plugins(WireWorld2dPlugin::default().with_time_step(1. / 30.))
        .insert_resource(SimulationBatch)
        .add_systems(Startup, (setup_camera, setup_map))
        .add_systems(Update, color_sprites)
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
            for (index, state) in state.cells.into_iter().enumerate() {
                if let Some(state) = state {
                    let (x, y) = (index as f32 % WIDTH, index as f32 / WIDTH);

                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(SCALE)),
                            ..default()
                        },
                        Transform::from_xyz(SCALE * x as f32, -SCALE * y as f32, 0.),
                        MooreCell2d::new(IVec2::new(x as i32, y as i32)),
                        state,
                    ));
                }
            }
        });
}

pub fn color_sprites(
    mut query: Query<(&WireWorldCellState, &mut Sprite), Changed<WireWorldCellState>>,
) {
    query
        .par_iter_mut()
        .for_each(|(state, mut sprite)| match state {
            WireWorldCellState::ElectronHead => sprite.color = Color::Srgba(BLUE),
            WireWorldCellState::ElectronTail => sprite.color = Color::Srgba(WHITE),
            WireWorldCellState::Conductor => sprite.color = Color::Srgba(ORANGE),
        });
}
