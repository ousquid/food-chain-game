pub mod components;
pub mod hp;
use crate::components::*;
use crate::hp::*;

use bevy::ecs::*;
use bevy::prelude::*;
use rand::prelude::*;
// https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/
use bevy_prototype_lyon::prelude::*;

const UNIT_WIDTH: u32 = 20;
const UNIT_HEIGHT: u32 = 20;

const FIELD_LEFTBTM_X: i32 = 1;
const FIELD_LEFTBTM_Y: i32 = 6;
const FIELD_WIDTH: u32 = 16;
const FIELD_HEIGHT: u32 = 28;

const SCREEN_WIDTH: u32 = 24;
const SCREEN_HEIGHT: u32 = 36;

const INITIAL_BEAR_NUM: u32 = 5;
const INITIAL_FOX_NUM: u32 = 8;
const INITIAL_WALNUT_NUM: u32 = 10;

const HEALING_STAMINA_HUMAN: i32 = 10;
const HEALING_STAMINA_BEAR: i32 = 8;
const HEALING_STAMINA_FOX: i32 = 5;
const HEALING_STAMINA_WALNUT: i32 = 0;
const MAX_STAMINA: i32 = 100;

#[derive(Component)]
struct State {
    kind: StateKind,
}

enum StateKind {
    GameOver,
    GameClear,
    Playing,
}

struct GameTimer(Timer);

#[derive(Component)]
pub struct Stamina {
    pub healing_val: i32,
    pub val: i32,
}

impl Stamina {
    fn cool_down(&mut self) {
        if self.val < MAX_STAMINA {
            self.val += self.healing_val;
        }
    }
    fn can_move(&self) -> bool {
        self.val >= MAX_STAMINA
    }
    fn human() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_HUMAN,
            val: 0,
        }
    }
    fn bear() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_BEAR,
            val: 0,
        }
    }
    fn fox() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_FOX,
            val: 0,
        }
    }
    fn walnut() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_WALNUT,
            val: 0,
        }
    }
}

fn get_random_direction() -> Position {
    let choices = [
        Position { x: -1, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 1 },
        Position { x: 0, y: 0 },
    ];
    let mut rng = thread_rng();
    *choices.choose(&mut rng).unwrap()
}

fn get_random_position() -> Position {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(FIELD_LEFTBTM_X..FIELD_LEFTBTM_X as i32 + FIELD_WIDTH as i32);
    let y = rng.gen_range(FIELD_LEFTBTM_Y..FIELD_LEFTBTM_Y as i32 + FIELD_HEIGHT as i32);
    Position { x, y }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "FoodChainGame".to_string(),
            width: (SCREEN_WIDTH * UNIT_WIDTH) as f32,
            height: (SCREEN_HEIGHT * UNIT_HEIGHT) as f32,
            ..Default::default()
        })
        .insert_resource(GameTimer(Timer::new(
            std::time::Duration::from_millis(100),
            true,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(HpPlugin)
        .add_startup_system(setup_system)
        .add_system(heal)
        .add_system(move_player)
        .add_system(move_fox)
        .add_system(move_bear)
        .add_system(text_value)
        .add_system(game_timer)
        .add_system(goal)
        .add_system(despawn_hp_text)
        .add_system(spawn_all_hp_text)
        .add_system(position_transform)
        .add_system(despawn.after("eaten"))
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    for i in 0..FIELD_WIDTH as i32 {
        for j in 0..FIELD_HEIGHT as i32 {
            spawn_field(
                &mut commands,
                Position {
                    x: i + FIELD_LEFTBTM_X,
                    y: j + FIELD_LEFTBTM_Y,
                },
            );
        }
    }
    spawn_terminal(
        &mut commands,
        Position {
            x: FIELD_WIDTH as i32 + FIELD_LEFTBTM_X as i32 - 1,
            y: FIELD_HEIGHT as i32 + FIELD_LEFTBTM_Y as i32 - 1,
        },
    );
    spawn_player(&mut commands, Position { x: 4, y: 6 }, &asset_server);
    for _ in 0..INITIAL_BEAR_NUM {
        spawn_bear(&mut commands, get_random_position(), &asset_server);
    }
    for _ in 0..INITIAL_FOX_NUM {
        spawn_fox(&mut commands, get_random_position(), &asset_server);
    }
    for _ in 0..INITIAL_WALNUT_NUM {
        spawn_walnut(&mut commands, get_random_position(), &asset_server);
    }
    spawn_text(
        &mut commands,
        Position { x: 10, y: 10 },
        StateKind::Playing,
        &asset_server,
    );
}

fn game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    timer.0.tick(time.delta());
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform)>) {
    let origin_x = UNIT_WIDTH as i32 / 2 - (SCREEN_WIDTH as i32 * UNIT_WIDTH as i32) / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - (SCREEN_HEIGHT as i32 * UNIT_HEIGHT as i32) / 2;
    position_query.iter_mut().for_each(|(pos, mut transform)| {
        transform.translation = Vec3::new(
            (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
            (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
            0.0,
        );
    });
}

fn text_value(mut state_query: Query<(&State, &mut Text)>) {
    state_query.iter_mut().for_each(|(state, mut text)| {
        text.sections[0].value = match state.kind {
            StateKind::GameOver => "GameOver!!!!".to_string(),
            StateKind::GameClear => "GameClear!".to_string(),
            StateKind::Playing => "Playing!".to_string(),
        }
    });
}

fn spawn_text(
    commands: &mut Commands,
    position: Position,
    state_kind: StateKind,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "unknown!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(State { kind: state_kind });
}

fn spawn_field(commands: &mut Commands, position: Position) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32),
        ..shapes::Rectangle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(position)
        .insert(Field);
}

fn spawn_terminal(commands: &mut Commands, position: Position) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32),
        ..shapes::Rectangle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GRAY),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(position)
        .insert(Terminal);
}

fn spawn_player(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::PURPLE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(Player)
        .insert(Human)
        .insert(WalnutEater)
        .insert(FoxEater)
        .insert(BearEater)
        .insert(position)
        .insert(Stamina::human())
        .insert(HP::human());
}

fn spawn_bear(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(148, 115, 91)),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(Bear)
        .insert(WalnutEater)
        .insert(FoxEater)
        .insert(HumanEater)
        .insert(position)
        .insert(Stamina::bear())
        .insert(HP::bear());
}

fn spawn_fox(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(Fox)
        .insert(WalnutEater)
        .insert(position)
        .insert(Stamina::fox())
        .insert(HP::fox());
}

fn spawn_walnut(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(Walnut)
        .insert(position)
        .insert(Stamina::walnut())
        .insert(HP::walnut());
}

fn reachable(field_query: &Query<&Position, With<Field>>, x: i32, y: i32) -> bool {
    field_query
        .iter()
        .any(|pos_field| x == pos_field.x && y == pos_field.y)
}

fn move_fox(
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut fox_query: Query<(&mut Position, &mut Stamina), (With<Fox>, Without<Field>)>,
) {
    if !timer.0.finished() {
        return;
    }
    fox_query.iter_mut().for_each(|(mut pos_fox, mut stamina)| {
        let dir = get_random_direction();
        if stamina.can_move() {
            if reachable(&field_query, pos_fox.x + dir.x, pos_fox.y + dir.y) {
                pos_fox.x += dir.x;
                pos_fox.y += dir.y;
                stamina.val = 0
            }
        }
    })
}

fn move_bear(
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut bear_query: Query<(&mut Position, &mut Stamina), (With<Bear>, Without<Field>)>,
) {
    if !timer.0.finished() {
        return;
    }
    bear_query
        .iter_mut()
        .for_each(|(mut pos_bear, mut stamina)| {
            let dir = get_random_direction();
            if stamina.can_move() {
                if reachable(&field_query, pos_bear.x + dir.x, pos_bear.y + dir.y) {
                    pos_bear.x += dir.x;
                    pos_bear.y += dir.y;
                    stamina.val = 0
                }
            }
        })
}

fn move_player(
    key_input: Res<Input<KeyCode>>,
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut player_query: Query<(&mut Position, &mut Stamina), (With<Player>, Without<Field>)>,
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

    player_query
        .iter_mut()
        .for_each(|(mut pos_player, mut stamina)| {
            if stamina.can_move() && (x != 0 || y != 0) {
                if reachable(&field_query, pos_player.x + x, pos_player.y + y) {
                    pos_player.x += x;
                    pos_player.y += y;
                    stamina.val = 0
                }
            }
        })
}

fn heal(timer: ResMut<GameTimer>, mut food_query: Query<&mut Stamina>) {
    if !timer.0.finished() {
        return;
    }

    food_query
        .iter_mut()
        .for_each(|mut stamina| stamina.cool_down())
}

fn goal(
    player_query: Query<&Position, With<Player>>,
    terminal_query: Query<&Position, With<Terminal>>,
    mut state_query: Query<&mut State>,
) {
    player_query.iter().for_each(|pos_player| {
        if terminal_query
            .iter()
            .any(|pos_field| pos_player.x == pos_field.x && pos_player.y == pos_field.y)
        {
            state_query.iter_mut().for_each(|mut state| {
                state.kind = StateKind::GameClear;
            });
        }
    })
}

fn despawn_hp_text(mut commands: Commands, text_query: Query<Entity, With<HpText>>) {
    text_query.iter().for_each(|text| {
        commands.entity(text).despawn();
    })
}

fn spawn_all_hp_text(
    mut commands: Commands,
    character_query: Query<(&Position, &HP)>,
    asset_server: Res<AssetServer>,
) {
    character_query
        .iter()
        .for_each(|(pos_character, hp_character)| {
            spawn_hp_text(&mut commands, pos_character, hp_character, &asset_server);
        })
}

fn spawn_hp_text(
    commands: &mut Commands,
    position: &Position,
    hp: &HP,
    asset_server: &Res<AssetServer>,
) {
    let text = Text::with_section(
        format!("{}", hp.val as i32),
        TextStyle {
            font_size: 10.0,
            color: Color::BLACK,
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        },
        Default::default(),
    );

    let origin_x = UNIT_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2;

    commands
        .spawn_bundle(TextBundle {
            text: text,
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px((origin_x + position.x * UNIT_WIDTH as i32) as f32),
                    bottom: Val::Px((origin_y + position.y * UNIT_HEIGHT as i32) as f32),
                    ..default()
                },
                ..default()
            },
            ..Default::default()
        })
        .insert(HpText);
}

fn despawn(mut commands: Commands, mut food_query: Query<(Entity, &HP)>) {
    food_query.iter_mut().for_each(|(entity, hp)| {
        if hp.val <= 0.0 {
            commands.entity(entity).despawn();
        }
    })
}
