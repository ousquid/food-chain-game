use std::f32::INFINITY;

// Display
pub const UNIT_WIDTH: u32 = 20;
pub const UNIT_HEIGHT: u32 = 20;

// Layer
pub const FIELD_LAYER: i32 = 0;
pub const TERMINAL_LAYER: i32 = 1;
pub const PLAYER_LAYER: i32 = 2;
pub const BEAR_LAYER: i32 = 3;
pub const TEXT_LAYER: i32 = 4;

// Field
pub const FIELD_LEFTBTM_X: i32 = 1;
pub const FIELD_LEFTBTM_Y: i32 = 6;
pub const FIELD_WIDTH: u32 = 16;
pub const FIELD_HEIGHT: u32 = 28;

pub const SCREEN_WIDTH: u32 = 24;
pub const SCREEN_HEIGHT: u32 = 36;

// Food Spawn
pub const INITIAL_BEAR_NUM: u32 = 1;
pub const INITIAL_FOX_NUM: u32 = 3;
pub const INITIAL_WALNUT_NUM: u32 = 10;

// probability (5% = 500)
pub const PROBABILITY_INCREASE_WALNUT: u32 = 300;

// Game Tick
// 1Game 3.5 min = 210 sec = 2100 tick
pub const GAME_DEFAULT_FPS: i32 = 10;
// pub const GAME_FPS: i32 = GAME_DEFAULT_FPS; // 100 / GAME_TICK as i32;
pub const GAME_FPS: i32 = 10;
pub const GAME_TICK: u64 = 1000 / GAME_FPS as u64; // 10; // ms

// Age
pub const HEALTHSPAN_STRONG_BEAR: i32 = 210 * GAME_DEFAULT_FPS; // 210 sec x 10 FPS
pub const LIFESPAN_WEAK_BEAR: i32 = 300 * GAME_DEFAULT_FPS; // 300 sec

// HP
// decrease hp 2.0 by sec
pub const HUNGRY_SPEED_BY_TICK: f32 = 2.0 / GAME_DEFAULT_FPS as f32;

pub const MAX_HP_HUMAN: f32 = 100.0;
pub const MAX_HP_BEAR: f32 = 30.0;
pub const MAX_HP_FOX: f32 = 5.0;
pub const MAX_HP_WALNUT: f32 = 1.0;

pub const DECREASE_HP_HUMAN: f32 = 1.0 / GAME_DEFAULT_FPS as f32; // eat fox -> 10sec eat bear-> 30sec
pub const DECREASE_HP_BEAR: f32 = 0.3 / GAME_DEFAULT_FPS as f32; // eat fox -> 30sec
pub const DECREASE_HP_FOX: f32 = 0.1 / GAME_DEFAULT_FPS as f32;
pub const DECREASE_HP_WALNUT: f32 = 0.0;

pub const INITIAL_HP_HUMAN: f32 = 50.0;

pub const HEALING_HP_HUMAN: f32 = 20.0;
pub const HEALING_HP_BEAR: f32 = 30.0;
pub const HEALING_HP_FOX: f32 = 5.0;
pub const HEALING_HP_WALNUT: f32 = 1.0;

// Stamina
pub const MAX_STAMINA: i32 = 100;

pub const HEALING_STAMINA_HUMAN: i32 = 30;
pub const HEALING_STAMINA_STRONG_BEAR: i32 = 10;
pub const HEALING_STAMINA_WEAK_BEAR: i32 = 10;
pub const HEALING_STAMINA_FOX: i32 = 10;
pub const HEALING_STAMINA_WALNUT: i32 = 0;
// MAX_STAMINA / HEALING_STAMINA_SHIP / GAME_TICK_BY_SEC  * move_array_size
// 100 / 4 / 10 * 120 = 300 sec = 5.0 min
pub const HEALING_STAMINA_SHIP: i32 = 4;

// Satiety
pub const MAX_WALNUT_COUNT: usize = 15;
pub const MAX_SATIETY_HUMAN: f32 = INFINITY;
pub const MAX_SATIETY_STRONG_BEAR: f32 = HEALING_SATIETY_FOX * 5.;
pub const MAX_SATIETY_WEAK_BEAR: f32 = INFINITY;
pub const MAX_SATIETY_FOX: f32 = HEALING_SATIETY_WALNUT / 2.;
pub const MAX_SATIETY_WALNUT: f32 = INFINITY;

pub const HEALING_SATIETY_HUMAN: f32 = 8.0;
pub const HEALING_SATIETY_BEAR: f32 = 4.0;
pub const HEALING_SATIETY_FOX: f32 = 2.0;
pub const HEALING_SATIETY_WALNUT: f32 = 1.0;
