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
    let (size_x, size_y) = (100, 100);
    let sprite_size = 2.;

    commands
        .spawn((
            Transform::from_xyz(
                -(size_x as f32 * sprite_size) / 2.,
                -(size_y as f32 * sprite_size) / 2.,
                0.,
            ),
            Visibility::default(),
        ))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    builder.spawn((
                        Sprite {
                            custom_size: Some(Vec2::splat(sprite_size)),
                            color: Color::srgba(255., 255., 255., 255.),
                            ..default()
                        },
                        Transform::from_xyz(sprite_size * x as f32, sprite_size * y as f32, 0.),
                    ));
                }
            }
        });
}
