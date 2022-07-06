use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;

const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "FoodChainGame".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 20.,
        center: Vec2::new(0.0, 0.0),
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PURPLE),
            outline_mode: StrokeMode::new(Color::BLACK, 0.0),
        },
        Transform::default(),
    ));
}