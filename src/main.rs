pub mod components;
pub mod consts;
pub mod eat;

use crate::components::*;
use crate::consts::*;
use crate::eat::*;
use array_macro::*;
use std::collections::HashSet;

use bevy::ecs::*;
use bevy::prelude::*;
use rand::prelude::*;
// https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/
use bevy_prototype_lyon::prelude::*;

pub const SHIP_MOVING: [Position; 88] = array![x => match x {
    0..=11 => Position::right(),
    12..=43 => Position::down(),
    44..=75 => Position::up(),
    76..=87 => Position::left(),
    _ => Position::stay(),
}; 88];

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
    fn strong_bear() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_STRONG_BEAR,
            val: 0,
        }
    }
    fn weak_bear() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_WEAK_BEAR,
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
    fn ship() -> Stamina {
        Stamina {
            healing_val: HEALING_STAMINA_SHIP,
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

fn get_increase_pos(pos: &Position, range: u32) -> Position {
    loop {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-(range as i32)..range as i32);
        let y = rng.gen_range(-(range as i32)..range as i32);
        let new_pos = Position { x, y };
        if new_pos != *pos {
            return new_pos;
        }
    }
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
            std::time::Duration::from_millis(GAME_TICK),
            true,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(HpPlugin)
        .add_startup_system(setup_system)
        .add_system(heal)
        .add_system(get_old)
        .add_system(move_player)
        .add_system(move_fox)
        .add_system(move_strong_bear)
        .add_system(move_weak_bear)
        .add_system(move_ship)
        .add_system(increase_strong_bear)
        .add_system(increase_fox)
        .add_system(increase_walnut)
        .add_system(text_value)
        .add_system(game_timer)
        .add_system(goal)
        .add_system(despawn_hp_text)
        .add_system(spawn_all_hp_text)
        .add_system(position_transform)
        .add_system(weaken_bear)
        .add_system(die_of_old_age)
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
    spawn_ship(
        &mut commands,
        Position {
            x: FIELD_WIDTH as i32 + FIELD_LEFTBTM_X as i32 - 1,
            y: FIELD_HEIGHT as i32 + FIELD_LEFTBTM_Y as i32 - 1,
        },
    );
    spawn_player(&mut commands, Position { x: 4, y: 6 }, &asset_server);
    for _ in 0..INITIAL_BEAR_NUM {
        spawn_strong_bear(
            &mut commands,
            get_random_position(),
            &asset_server,
            MAX_HP_BEAR,
        );
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

fn get_render_position(pos: &Position) -> Position {
    let origin_x = UNIT_WIDTH as i32 / 2 - (SCREEN_WIDTH as i32 * UNIT_WIDTH as i32) / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - (SCREEN_HEIGHT as i32 * UNIT_HEIGHT as i32) / 2;
    return Position {
        x: origin_x + pos.x as i32 * UNIT_WIDTH as i32,
        y: origin_y + pos.y as i32 * UNIT_HEIGHT as i32,
    };
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform)>) {
    position_query.iter_mut().for_each(|(pos, mut transform)| {
        let render_pos = get_render_position(pos);
        transform.translation = Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0);
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
                fill_mode: FillMode::color(Color::rgb(0.7, 0.7, 1.0)),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(position)
        .insert(Terminal);
}

fn spawn_ship(commands: &mut Commands, position: Position) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32),
        ..shapes::Rectangle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform::default(),
        ))
        .insert(position)
        .insert(Ship { index: 0 })
        .insert(Stamina::ship());
}

fn spawn_player(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };
    let render_pos = get_render_position(&position);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::PURPLE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0),
                ..Default::default()
            },
        ))
        .insert(Player)
        .insert(Human)
        .insert(WalnutEater)
        .insert(FoxEater)
        .insert(WeakBearEater)
        .insert(StrongBearPrey)
        .insert(position)
        .insert(Stamina::human())
        .insert(HP::human())
        .insert(Satiety::human());
}

fn spawn_strong_bear(
    commands: &mut Commands,
    position: Position,
    asset_server: &Res<AssetServer>,
    hp: f32,
) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };
    let render_pos = get_render_position(&position);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(148, 115, 91)),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0),
                ..Default::default()
            },
        ))
        .insert(StrongBear)
        .insert(WalnutEater)
        .insert(FoxEater)
        .insert(HumanEater)
        .insert(position)
        .insert(Stamina::strong_bear())
        .insert(HP::bear(hp))
        .insert(Satiety::strong_bear())
        .insert(Age { val: 0 });
}

fn spawn_weak_bear(
    commands: &mut Commands,
    position: Position,
    asset_server: &Res<AssetServer>,
    hp: f32,
) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };
    let render_pos = get_render_position(&position);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(148, 115, 255)),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0),
                ..Default::default()
            },
        ))
        .insert(WeakBear)
        .insert(WalnutEater)
        .insert(FoxEater)
        .insert(HumanPrey)
        .insert(position)
        .insert(Stamina::weak_bear())
        .insert(HP::bear(hp))
        .insert(Satiety::weak_bear())
        .insert(Age {
            val: HEALTHSPAN_STRONG_BEAR,
        });
}

fn spawn_fox(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };
    let render_pos = get_render_position(&position);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0),
                ..Default::default()
            },
        ))
        .insert(Fox)
        .insert(WalnutEater)
        .insert(StrongBearPrey)
        .insert(WeakBearPrey)
        .insert(HumanPrey)
        .insert(position)
        .insert(Stamina::fox())
        .insert(HP::fox())
        .insert(Satiety::fox());
}

fn spawn_walnut(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let shape = shapes::Circle {
        radius: (UNIT_WIDTH / 2) as f32,
        center: Vec2::new(0.0, 0.0),
    };
    let render_pos = get_render_position(&position);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::BLACK, 0.0),
            },
            Transform {
                translation: Vec3::new(render_pos.x as f32, render_pos.y as f32, 0.0),
                ..Default::default()
            },
        ))
        .insert(Walnut)
        .insert(StrongBearPrey)
        .insert(WeakBearPrey)
        .insert(HumanPrey)
        .insert(FoxPrey)
        .insert(position)
        .insert(Stamina::walnut())
        .insert(HP::walnut())
        .insert(Satiety::walnut());
}

fn increase_walnut(
    mut commands: Commands,
    timer: ResMut<GameTimer>,
    walnut_query: Query<&Position, With<Walnut>>,
    field_query: Query<&Position, With<Field>>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let mut new_walnuts: HashSet<Position> = HashSet::new();
    walnut_query.iter().for_each(|position| {
        if rng.gen_range(1..=100) > 95 {
            let offset = get_increase_pos(&position, 2);
            let new_pos = Position {
                x: position.x + offset.x,
                y: position.y + offset.y,
            };
            if reachable(&field_query, new_pos.x, new_pos.y) {
                new_walnuts.insert(new_pos);
            }
        }
    });
    walnut_query.iter().for_each(|position| {
        new_walnuts.remove(position);
    });
    for pos in new_walnuts {
        spawn_walnut(&mut commands, pos, &asset_server);
    }
}

fn increase_fox(
    mut commands: Commands,
    timer: ResMut<GameTimer>,
    mut fox_query: Query<(&Position, &mut Satiety), With<Fox>>,
    field_query: Query<&Position, With<Field>>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.finished() {
        return;
    }
    fox_query.iter_mut().for_each(|(position, mut satiety)| {
        if satiety.val >= satiety.max {
            satiety.val -= satiety.max;
            let offset = get_increase_pos(&position, 2);
            let new_pos = Position {
                x: position.x + offset.x,
                y: position.y + offset.y,
            };
            if reachable(&field_query, new_pos.x, new_pos.y) {
                spawn_fox(&mut commands, new_pos, &asset_server);
            }
        }
    })
}

fn increase_strong_bear(
    mut commands: Commands,
    timer: ResMut<GameTimer>,
    mut strong_bear_query: Query<(&Position, &mut Satiety), With<StrongBear>>,
    field_query: Query<&Position, With<Field>>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.finished() {
        return;
    }
    strong_bear_query
        .iter_mut()
        .for_each(|(position, mut satiety)| {
            if satiety.val >= satiety.max {
                satiety.val -= satiety.max;
                let offset = get_increase_pos(&position, 2);
                let new_pos = Position {
                    x: position.x + offset.x,
                    y: position.y + offset.y,
                };
                if reachable(&field_query, new_pos.x, new_pos.y) {
                    spawn_strong_bear(&mut commands, new_pos, &asset_server, MAX_HP_BEAR);
                }
            }
        })
}

fn reachable(field_query: &Query<&Position, With<Field>>, x: i32, y: i32) -> bool {
    field_query
        .iter()
        .any(|pos_field| x == pos_field.x && y == pos_field.y)
}

fn distance(pos1: &Position, pos2: &Position) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}
fn get_neighbor<'a, T>(prey: T, pos: &Position) -> Option<&'a Position>
where
    T: Iterator<Item = &'a Position>,
{
    prey.min_by_key(|pos_food| distance(pos_food, pos))
}
fn approach<'a, T>(prey: T, pos: &Position, field_query: &Query<&Position, With<Field>>) -> Position
where
    T: Iterator<Item = &'a Position>,
{
    let new_positions = [
        Position { x: 1, y: 0 },
        Position { x: -1, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 0, y: -1 },
    ]
    .iter()
    .map(|dir| pos + dir);

    if let Some(neighbor) = get_neighbor(prey, &pos) {
        let mut rng = thread_rng();
        if rng.gen_range(1..=10) <= 7 {
            new_positions
                .filter(|p| reachable(&field_query, p.x, p.y))
                .min_by_key(|p| distance(p, neighbor))
                .unwrap_or(Position { x: pos.x, y: pos.y })
        } else {
            &(get_random_direction()) + pos
        }
    } else {
        &(get_random_direction()) + pos
    }
}
fn move_fox(
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut fox_query: Query<
        (&mut Position, &mut Stamina),
        (With<Fox>, Without<Field>, Without<FoxPrey>),
    >,
    prey_query: Query<&Position, With<FoxPrey>>,
) {
    if !timer.0.finished() {
        return;
    }
    fox_query.iter_mut().for_each(|(mut pos_fox, mut stamina)| {
        if stamina.can_move() {
            let decided_pos = approach(prey_query.iter(), &pos_fox, &field_query);
            pos_fox.x = decided_pos.x;
            pos_fox.y = decided_pos.y;
            stamina.val = 0
        }
    })
}

fn move_strong_bear(
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut strong_bear_query: Query<
        (&mut Position, &mut Stamina),
        (With<StrongBear>, Without<Field>, Without<StrongBearPrey>),
    >,
    prey_query: Query<&Position, With<StrongBearPrey>>,
) {
    if !timer.0.finished() {
        return;
    }
    strong_bear_query
        .iter_mut()
        .for_each(|(mut pos_strong_bear, mut stamina)| {
            if stamina.can_move() {
                let decided_pos = approach(prey_query.iter(), &pos_strong_bear, &field_query);
                pos_strong_bear.x = decided_pos.x;
                pos_strong_bear.y = decided_pos.y;
                stamina.val = 0
            }
        })
}

fn move_weak_bear(
    timer: ResMut<GameTimer>,
    field_query: Query<&Position, With<Field>>,
    mut weak_bear_query: Query<
        (&mut Position, &mut Stamina),
        (With<WeakBear>, Without<Field>, Without<WeakBearPrey>),
    >,
    prey_query: Query<&Position, With<WeakBearPrey>>,
) {
    if !timer.0.finished() {
        return;
    }
    weak_bear_query
        .iter_mut()
        .for_each(|(mut pos_weak_bear, mut stamina)| {
            if stamina.can_move() {
                let decided_pos = approach(prey_query.iter(), &pos_weak_bear, &field_query);
                pos_weak_bear.x = decided_pos.x;
                pos_weak_bear.y = decided_pos.y;
                stamina.val = 0
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

fn heal(timer: ResMut<GameTimer>, mut query: Query<&mut Stamina>) {
    if !timer.0.finished() {
        return;
    }

    query.iter_mut().for_each(|mut stamina| stamina.cool_down())
}

fn get_old(timer: ResMut<GameTimer>, mut query: Query<&mut Age>) {
    if !timer.0.finished() {
        return;
    }

    query.iter_mut().for_each(|mut age| age.val += 1)
}

fn goal(
    mut commands: Commands,
    player_query: Query<(Entity, &Position), With<Player>>,
    ship_query: Query<&Position, With<Ship>>,
    mut state_query: Query<&mut State>,
) {
    player_query.iter().for_each(|(player, pos_player)| {
        if ship_query
            .iter()
            .any(|pos_ship| pos_player.x == pos_ship.x && pos_player.y == pos_ship.y)
        {
            state_query.iter_mut().for_each(|mut state| {
                state.kind = StateKind::GameClear;
            });
            commands.entity(player).despawn();
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

fn move_ship(
    timer: ResMut<GameTimer>,
    mut ship_query: Query<(&mut Ship, &mut Position, &mut Stamina)>,
) {
    if !timer.0.finished() {
        return;
    }

    ship_query
        .iter_mut()
        .for_each(|(mut ship, mut pos, mut stamina)| {
            if stamina.can_move() {
                let offset = SHIP_MOVING[ship.index];

                pos.x += offset.x;
                pos.y += offset.y;
                stamina.val = 0;
                ship.index = (ship.index + 1) % SHIP_MOVING.len()
            }
        })
}

fn weaken_bear(
    mut commands: Commands,
    mut strong_bear_query: Query<(Entity, &Position, &HP, &Age), With<StrongBear>>,
    asset_server: Res<AssetServer>,
) {
    strong_bear_query
        .iter_mut()
        .for_each(|(strong_bear, pos, hp, age)| {
            if age.val >= HEALTHSPAN_STRONG_BEAR {
                commands.entity(strong_bear).despawn();
                spawn_weak_bear(&mut commands, *pos, &asset_server, hp.val);
            }
        });
}

fn die_of_old_age(
    mut commands: Commands,
    mut weak_bear_query: Query<(Entity, &Age), With<WeakBear>>,
) {
    weak_bear_query.iter_mut().for_each(|(weak_bear, age)| {
        if age.val >= LIFESPAN_WEAK_BEAR {
            commands.entity(weak_bear).despawn();
        }
    });
}
