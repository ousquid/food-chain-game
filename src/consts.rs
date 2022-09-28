use std::f32::INFINITY;

// Display
pub const UNIT_WIDTH: u32 = 20;
pub const UNIT_HEIGHT: u32 = 20;

// Field
pub const FIELD_LEFTBTM_X: i32 = 1;
pub const FIELD_LEFTBTM_Y: i32 = 6;
pub const FIELD_WIDTH: u32 = 16;
pub const FIELD_HEIGHT: u32 = 28;

pub const SCREEN_WIDTH: u32 = 24;
pub const SCREEN_HEIGHT: u32 = 36;

// Food Spawn
pub const INITIAL_BEAR_NUM: u32 = 20;
pub const INITIAL_FOX_NUM: u32 = 1;
pub const INITIAL_WALNUT_NUM: u32 = 0;

// Game Tick
// 1Game 3.5 min = 210 sec = 2100 tick
pub const GAME_TICK: u64 = 100; // ms
pub const GAME_TICKS_BY_SEC: i32 = 1000 / GAME_TICK as i32;

// Age
pub const HEALTHSPAN_STRONG_BEAR: i32 = 20 * GAME_TICKS_BY_SEC;
pub const LIFESPAN_WEAK_BEAR: i32 = 30 * GAME_TICKS_BY_SEC;

// HP
pub const HUNGRY_SPEED_BY_TICK: f32 = 2.0 / GAME_TICK as f32;

pub const MAX_HP_HUMAN: f32 = 60.0;
pub const MAX_HP_BEAR: f32 = 300.0;
pub const MAX_HP_FOX: f32 = 50.0;
pub const MAX_HP_WALNUT: f32 = INFINITY;
pub const WEAK_BEAR_HP_THRESHOLD: f32 = 270.0;

pub const HEALING_HP_HUMAN: f32 = 30.0;
pub const HEALING_HP_BEAR: f32 = 60.0;
pub const HEALING_HP_FOX: f32 = 10.0;
pub const HEALING_HP_WALNUT: f32 = 5.0;

// Stamina
pub const MAX_STAMINA: i32 = 100;

pub const HEALING_STAMINA_HUMAN: i32 = 30;
pub const HEALING_STAMINA_STRONG_BEAR: i32 = 8;
pub const HEALING_STAMINA_WEAK_BEAR: i32 = 8;
pub const HEALING_STAMINA_FOX: i32 = 10;
pub const HEALING_STAMINA_WALNUT: i32 = 0;
pub const HEALING_STAMINA_SHIP: i32 = 4; // 2.5sec / 1move * 88 = 220 sec = 3.5 min
