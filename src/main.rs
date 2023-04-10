use bevy::prelude::*;
use rand::prelude::random;


const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct Food; 

#[derive(Resource)]
struct FoodSpawnTimer(Timer);

#[derive(Resource)]
struct BTimer(Timer);

#[derive(PartialEq, Copy, Clone)]
enum Direction { 
    Left,
    Up,
    Right,
    Down,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}



fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
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
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

// Query is doing some magic to iterate over all entities with both the SnakeHead and
// Transform components
// the previous version of the query Query<(&SnakeHead, &mut Transform)> returned an
// iterator containing both the snake head and the Transform compnent. We don't need the
// snake head so we've discarded it in the current version
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}


fn food_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>, 
    ) {
    
    // This seems expensive... checking every time if the timer has finished. Is it?
    if !timer.0.tick(time.delta()).finished() {
        return; 
    }

    commands
    .spawn(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(Food)
    .insert(Position { // it seem like there should be a better way to do this..
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_WIDTH as f32) as i32,
    })
    .insert(Size::square(0.8)); 
}
// size_scaling and position translation scale the board based on the size of the window and the
// board size constants
fn size_scaling(
    primary_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = primary_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    primary_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = primary_query.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .insert_resource(BTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_system(food_spawner.in_schedule(CoreSchedule::FixedUpdate))
        .add_systems((position_translation, size_scaling).chain())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: bevy::window::WindowResolution::new(500.0, 500.0),
                ..default()
            }),
            ..default()
        }))
        .run();
}