use bevy::prelude::*;

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
    let (width, height) = (200, 100);
    let sprite_size = 2.;

    let mut cells = ndarray::Array::zeros((width, height));
    cells[(50, 50)] = 1;

    commands
        .spawn((
            Transform::from_xyz(
                -(width as f32 * sprite_size) / 2.,
                -(height as f32 * sprite_size) / 2.,
                0.,
            ),
            Visibility::default(),
        ))
        .with_children(|builder| {
            cells
                .indexed_iter()
                .for_each(|((x, y), state): ((usize, usize), &u32)| {
                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(sprite_size)),
                            color: [
                                Color::srgba(0., 0., 0., 255.),
                                Color::srgba(255., 255., 255., 255.),
                            ][*state as usize],
                            ..default()
                        },
                        Transform::from_xyz(sprite_size * x as f32, sprite_size * y as f32, 0.),
                    ));
                });
        });
}
