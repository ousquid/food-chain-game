use std::f32::INFINITY;

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

const MOVE_SPEED_HUMAN: u32 = 10;
const MOVE_SPEED_BEAR: u32 = 8;
const MOVE_SPEED_FOX: u32 = 5;
const MOVE_SPEED_WALNUT: u32 = 0;

const MAX_HP_HUMAN: f32 = 60.0;
const MAX_HP_BEAR: f32 = 300.0;
const MAX_HP_FOX: f32 = 30.0;
const MAX_HP_WALNUT: f32 = INFINITY;

const HEALING_HP_HUMAN: f32 = 30.0;
const HEALING_HP_BEAR: f32 = 60.0;
const HEALING_HP_FOX: f32 = 10.0;
const HEALING_HP_WALNUT: f32 = 5.0;

const WEAK_HP_RATIO: f32 = 0.9;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}
struct GameTimer(Timer);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Human;
#[derive(Component)]
struct Bear;
#[derive(Component)]
struct Fox;
#[derive(Component)]
struct Walnut;

#[derive(Component)]
struct WalnutEater;
#[derive(Component)]
struct FoxEater;
#[derive(Component)]
struct BearEater;
#[derive(Component)]
struct HumanEater;

#[derive(Component)]
struct Field;
#[derive(Component)]
struct Terminal;

#[derive(Component)]
struct HpText;

#[derive(Component)]
struct State {
    kind: StateKind,
}
#[derive(Component)]
struct Counter {
    val: i32,
}
#[derive(Component)]
struct HP {
    val: f32,
    max: f32,
}

enum StateKind {
    GameOver,
    GameClear,
    Playing,
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
        .add_startup_system(setup_system)
        .add_system(move_player)
        .add_system(text_value)
        .add_system(game_timer)
        .add_system(goal)
        .add_system(despawn_hp_text)
        .add_system(spawn_all_hp_text)
        .add_system(position_transform)
        .add_system(hungry)
        .add_system_set(
            SystemSet::new()
            .label("eat")
            .with_system(eat_walnut)
            .with_system(eat_fox)
            .with_system(eat_bear)
            .with_system(eat_human)
        )
        .add_system_set(
            SystemSet::new()
            .after("eat")
            .label("eaten")
            .with_system(eaten_walnut)
            .with_system(eaten_fox)
            .with_system(eaten_bear)
            .with_system(eaten_human)
        )
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
    spawn_bear(&mut commands, get_random_position(), &asset_server);
    spawn_fox(&mut commands, get_random_position(), &asset_server);
    spawn_walnut(&mut commands, get_random_position(), &asset_server);
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
        .insert(HP {
            max: MAX_HP_HUMAN,
            val: MAX_HP_HUMAN,
        });
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
        .insert(HP {
            max: MAX_HP_BEAR,
            val: MAX_HP_BEAR,
        });
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
        .insert(HP {
            max: MAX_HP_FOX,
            val: MAX_HP_FOX,
        });
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
        .insert(HP {
            max: MAX_HP_WALNUT,
            val: MAX_HP_WALNUT,
        });
}

fn move_player(
    key_input: Res<Input<KeyCode>>,
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut player_query: Query<&mut Position, (With<Player>, Without<Field>)>,
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

    player_query.iter_mut().for_each(|mut pos_player| {
        if field_query
            .iter()
            .any(|pos_field| pos_player.x + x == pos_field.x && pos_player.y + y == pos_field.y)
        {
            pos_player.x += x;
            pos_player.y += y;
        }
    })
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

/*
 * Walnut: すでにあるWalnutの付近にランダム生成。死なない。
 * Fox   : Walnut食べないと死ぬ。Walnut食べてたら増える。
 * Bear  : Fox or Walnut or Player食べないと死ぬ。Fox or Walnut食べてたら増える。
 * Player: Walnut or Fox or Bear食べないと死ぬ。Bearを食べないと生態系が崩れる仕様。
 *
 * 寿命
 * Ship: 5分に1回来る。30秒ぐらい滞在
 * Player: 止まってたら1分ぐらいで死ぬ。島を頑張って回る必要あり。島は端から端まで10秒ぐらいで移動可?
 */
fn hungry(
    mut commands: Commands,
    timer: ResMut<GameTimer>,
    mut food_query: Query<(Entity, &mut HP)>,
) {
    if !timer.0.finished() {
        return;
    }

    food_query.iter_mut().for_each(|(entity, mut hp)| {
        hp.val -= 0.2;
    })
}

fn eaten_walnut(
    eater_query: Query<(Entity, &Position, &HP), With<WalnutEater>>,
    mut walnut_query: Query<(Entity, &Position, &mut HP), (With<Walnut>, Without<WalnutEater>)>
) {
    walnut_query.iter_mut().for_each(|(_, w_pos, mut w_hp)| {
        eater_query.iter().for_each(|(_, e_pos, _)| {
            if e_pos == w_pos {
                w_hp.val = 0.0;
            }
        });
    });
    
}

fn eat_walnut(
    mut eater_query: Query<(Entity, &Position, &mut HP), (With<WalnutEater>, Without<Walnut>)>,
    walnut_query: Query<(Entity, &Position, &HP), With<Walnut>>
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        walnut_query.iter().for_each(|(_, w_pos, _)| {
            if e_pos == w_pos {
                e_hp.val +=  HEALING_HP_WALNUT;           
            }
        })
    })
}

fn eaten_fox(
    eater_query: Query<(Entity, &Position, &HP), With<FoxEater>>,
    mut fox_query: Query<(Entity, &Position, &mut HP), (With<Fox>, Without<FoxEater>)>
) {
    fox_query.iter_mut().for_each(|(_, f_pos, mut f_hp)| {
        eater_query.iter().for_each(|(_, e_pos, _)| {
            if e_pos == f_pos {
                f_hp.val = 0.0;
            }
        });
    });
    
}

fn eat_fox(
    mut eater_query: Query<(Entity, &Position, &mut HP), (With<FoxEater>, Without<Fox>)>,
    fox_query: Query<(Entity, &Position, &HP), With<Fox>>
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        fox_query.iter().for_each(|(_, f_pos, _)| {
            if e_pos == f_pos {
                e_hp.val +=  HEALING_HP_FOX;           
            }
        })
    })
}

fn eaten_bear(
    eater_query: Query<(Entity, &Position, &HP), With<BearEater>>,
    mut bear_query: Query<(Entity, &Position, &mut HP), (With<Bear>, Without<BearEater>)>
) {
    bear_query.iter_mut().for_each(|(_, b_pos, mut b_hp)| {
        eater_query.iter().for_each(|(_, e_pos, _)| {
            if b_pos == e_pos && b_hp.val < b_hp.max * WEAK_HP_RATIO {
                b_hp.val = 0.0;
            }
        });
    });
    
}

fn eat_bear(
    mut eater_query: Query<(Entity, &Position, &mut HP), (With<BearEater>, Without<Bear>)>,
    bear_query: Query<(Entity, &Position, &HP), With<Bear>>
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        bear_query.iter().for_each(|(_, b_pos, b_hp)| {
            if b_pos == e_pos && b_hp.val < b_hp.max * WEAK_HP_RATIO {
                e_hp.val +=  HEALING_HP_BEAR;           
            }
        })
    })
}

fn eaten_human(
    eater_query: Query<(Entity, &Position, &HP), With<HumanEater>>,
    mut human_query: Query<(Entity, &Position, &mut HP), (With<Human>, Without<HumanEater>)>
) {
    human_query.iter_mut().for_each(|(_, h_pos, mut h_hp)| {
        eater_query.iter().for_each(|(_, e_pos, e_hp)| {
            if h_pos == e_pos && e_hp.val > e_hp.max * WEAK_HP_RATIO {
                h_hp.val = 0.0;
            }
        });
    });
    
}

fn eat_human(
    mut eater_query: Query<(Entity, &Position, &mut HP), (With<HumanEater>, Without<Human>)>,
    human_query: Query<(Entity, &Position, &HP), With<Human>>
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        human_query.iter().for_each(|(_, h_pos, h_hp)| {
            if h_pos == e_pos && e_hp.val > e_hp.max * WEAK_HP_RATIO {
                e_hp.val +=  HEALING_HP_HUMAN;           
            }
        })
    })
}

fn despawn(
    mut commands: Commands,
    mut food_query: Query<(Entity, &HP)>,
){
    food_query.iter_mut().for_each(|(entity, hp)| {
        if hp.val <= 0.0 {
            commands.entity(entity).despawn();
        }
    })
}
