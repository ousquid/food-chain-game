use crate::consts::*;

use bevy::prelude::*;
use std::f32::INFINITY;
use std::ops;

#[derive(Component, Clone, Copy, Eq, Debug, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub const fn up() -> Position {
        Position { x: 0, y: 1, z: 0 }
    }
    pub const fn down() -> Position {
        Position { x: 0, y: -1, z: 0 }
    }
    pub const fn left() -> Position {
        Position { x: -1, y: 0, z: 0 }
    }
    pub const fn right() -> Position {
        Position { x: 1, y: 0, z: 0 }
    }
    pub const fn stay() -> Position {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl<'a, 'b> ops::Add<&'b Position> for &'a Position {
    type Output = Position;
    fn add(self, rhs: &'b Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
pub struct GameTimer(Timer);

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Human;
#[derive(Component)]
pub struct StrongBear;
#[derive(Component)]
pub struct WeakBear;
#[derive(Component)]
pub struct Fox;
#[derive(Component)]
pub struct Walnut;

#[derive(Component)]
pub struct WalnutEater;
#[derive(Component)]
pub struct FoxEater;
#[derive(Component)]
pub struct StrongBearEater;
#[derive(Component)]
pub struct WeakBearEater;
#[derive(Component)]
pub struct HumanEater;

#[derive(Component)]
pub struct WalnutPrey;
#[derive(Component)]
pub struct FoxPrey;
#[derive(Component)]
pub struct StrongBearPrey;
#[derive(Component)]
pub struct WeakBearPrey;
#[derive(Component)]
pub struct HumanPrey;

#[derive(Component)]
pub struct Field;
#[derive(Component)]
pub struct Terminal;
#[derive(Component)]
pub struct Ship {
    pub index: usize,
}

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
    pub decrease: f32,
}

#[derive(Component)]
pub struct Satiety {
    pub val: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Age {
    pub val: i32,
}

impl HP {
    pub fn human() -> HP {
        return HP {
            max: MAX_HP_HUMAN,
            val: INITIAL_HP_HUMAN,
            decrease: DECREASE_HP_HUMAN,
        };
    }
    pub fn bear(val: f32) -> HP {
        return HP {
            max: MAX_HP_BEAR,
            val,
            decrease: DECREASE_HP_BEAR,
        };
    }
    pub fn fox() -> HP {
        return HP {
            max: MAX_HP_FOX,
            val: MAX_HP_FOX,
            decrease: DECREASE_HP_FOX,
        };
    }
    pub fn walnut() -> HP {
        return HP {
            max: MAX_HP_WALNUT,
            val: MAX_HP_WALNUT,
            decrease: DECREASE_HP_WALNUT,
        };
    }
}

impl Satiety {
    pub fn human() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_HUMAN,
            val: 0.0,
        };
    }
    pub fn strong_bear() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_STRONG_BEAR,
            val: 0.0,
        };
    }
    pub fn weak_bear() -> Satiety {
        return Satiety {
            max: MAX_SATIETY_WEAK_BEAR,
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
