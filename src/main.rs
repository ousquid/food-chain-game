use bevy::prelude::*;
// https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/
use bevy_prototype_lyon::prelude::*;

const UNIT_WIDTH: u32 = 20;
const UNIT_HEIGHT: u32 = 20;

const FIELD_CENTER_X: i32 = 8;
const FIELD_CENTER_Y: i32 = 18;
const FIELD_WIDTH: u32 = 16;
const FIELD_HEIGHT: u32 = 28;

const SCREEN_WIDTH: u32 = 24;
const SCREEN_HEIGHT: u32 = 36;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct InputTimer(Timer);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "FoodChainGame".to_string(),
            width: (SCREEN_WIDTH * UNIT_WIDTH) as f32,
            height: (SCREEN_HEIGHT * UNIT_HEIGHT) as f32,
            ..Default::default()
        })
        .insert_resource(InputTimer(Timer::new(
            std::time::Duration::from_millis(100),
            true,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(move_player)
        .add_system(position_transform)
        .add_system(game_timer)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    spawn_field(&mut commands, Position { x: FIELD_CENTER_X, y: FIELD_CENTER_Y });
    spawn_player(&mut commands, Position { x: 4, y: 6 })
}

fn game_timer(
    time: Res<Time>,
    mut imput_timer: ResMut<InputTimer>,
) {
    imput_timer.0.tick(time.delta());
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform)>) {
    let origin_x = UNIT_WIDTH as i32 / 2 - (SCREEN_WIDTH as i32 * UNIT_WIDTH as i32) / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - (SCREEN_HEIGHT as i32 * UNIT_HEIGHT as i32) / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform)| {
            println!("{:?}", pos);
            transform.translation = Vec3::new(
                (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
        });
}

fn spawn_field(
    commands: &mut Commands,
    position: Position,
) {
    let shape = shapes::Rectangle {
        extents: Vec2::new((FIELD_WIDTH * UNIT_WIDTH) as f32, (FIELD_HEIGHT * UNIT_HEIGHT) as f32),
        ..shapes::Rectangle::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::GREEN),
            outline_mode: StrokeMode::new(Color::BLACK, 0.0),
        },
        Transform::default(),
    )).insert(position);
}

fn spawn_player(
    commands: &mut Commands,
    position: Position
) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::BLACK, 0.0),
        },
        Transform::default(),
    )).insert(position).insert(Player);
}

fn move_player(
    key_input: Res<Input<KeyCode>>,
    timer: ResMut<InputTimer>,
    mut position_query: Query<(Entity, &mut Position, &Player)>
) {
    if !timer.0.finished() {
        return;
    }
    
    let mut x = 0;
    let mut y = 0;
    if key_input.pressed(KeyCode::Left) {
        x -= 1;
    } 
    if key_input.pressed(KeyCode::Right) {
        x += 1;
    }
    if key_input.pressed(KeyCode::Up) {
        y += 1;
    }
    if key_input.pressed(KeyCode::Down) {
        y -= 1;   
    }

    position_query.iter_mut().for_each(|(_, mut pos, _)| {
        pos.x += x;
        pos.y += y;
    })

}