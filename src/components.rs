use bevy::prelude::*;

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
