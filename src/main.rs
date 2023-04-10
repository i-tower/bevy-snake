use bevy::prelude::*;

fn main() {
    App::new()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_snake)
    .add_system(snake_movement)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn spawn_snake(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        transform: Transform { 
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    })
    .insert(SnakeHead);
}

// Query is doing some magic to iterate over all entities with both the SnakeHead and
// Transform components
fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    for (_head, mut transform) in head_positions.iter_mut() {
        transform.translation.y += 2.;
    }
}