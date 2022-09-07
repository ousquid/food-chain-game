use bevy::prelude::*;
use std::f32::INFINITY;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
pub struct GameTimer(Timer);

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Human;
#[derive(Component)]
pub struct Bear;
#[derive(Component)]
pub struct Fox;
#[derive(Component)]
pub struct Walnut;

#[derive(Component)]
pub struct WalnutEater;
#[derive(Component)]
pub struct FoxEater;
#[derive(Component)]
pub struct BearEater;
#[derive(Component)]
pub struct HumanEater;

#[derive(Component)]
pub struct Field;
#[derive(Component)]
pub struct Terminal;

#[derive(Component)]
pub struct HpText;

#[derive(Component)]
pub struct Counter {
    pub val: i32,
}

#[derive(Component)]
pub struct HP {
    pub val: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Satiety {
    pub val: f32,
    pub max: f32,
}

const MAX_HP_HUMAN: f32 = 60.0;
const MAX_HP_BEAR: f32 = 300.0;
const MAX_HP_FOX: f32 = 30.0;
const MAX_HP_WALNUT: f32 = INFINITY;

pub const HEALING_HP_HUMAN: f32 = 30.0;
pub const HEALING_HP_BEAR: f32 = 60.0;
pub const HEALING_HP_FOX: f32 = 10.0;
pub const HEALING_HP_WALNUT: f32 = 5.0;

pub const WEAK_HP_RATIO: f32 = 0.9;

impl HP {
    pub fn human() -> HP {
        return HP {
            max: MAX_HP_HUMAN,
            val: MAX_HP_HUMAN,
        };
    }
    pub fn bear() -> HP {
        return HP {
            max: MAX_HP_BEAR,
            val: MAX_HP_BEAR,
        };
    }
    pub fn fox() -> HP {
        return HP {
            max: MAX_HP_FOX,
            val: MAX_HP_FOX,
        };
    }
    pub fn walnut() -> HP {
        return HP {
            max: MAX_HP_WALNUT,
            val: MAX_HP_WALNUT,
        };
    }
}

const MAX_SATIETY_HUMAN: f32 = INFINITY;
const MAX_SATIETY_BEAR: f32 = 10.0;
const MAX_SATIETY_FOX: f32 = 5.0;
const MAX_SATIETY_WALNUT: f32 = INFINITY;

pub const HEALING_SATIETY_HUMAN: f32 = 8.0;
pub const HEALING_SATIETY_BEAR: f32 = 4.0;
pub const HEALING_SATIETY_FOX: f32 = 2.0;
pub const HEALING_SATIETY_WALNUT: f32 = 1.0;

impl Satiety {
    pub fn human() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_HUMAN,
            val: 0.0,
        };
    }
    pub fn bear() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_BEAR,
            val: 0.0,
        };
    }
    pub fn fox() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_FOX,
            val: 0.0,
        };
    }
    pub fn walnut() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_WALNUT,
            val: 0.0,
        };
    }
}
