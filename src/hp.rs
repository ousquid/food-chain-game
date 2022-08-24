use crate::components::*;

use bevy::prelude::*;
use std::f32::INFINITY;

use bevy::ecs::*;
use bevy::prelude::*;
use rand::prelude::*;
// https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/
use bevy_prototype_lyon::prelude::*;
pub struct HpPlugin;
struct StomachTimer(Timer);

#[derive(Component)]
pub struct HP {
    pub val: f32,
    pub max: f32,
}

const MAX_HP_HUMAN: f32 = 60.0;
const MAX_HP_BEAR: f32 = 300.0;
const MAX_HP_FOX: f32 = 30.0;
const MAX_HP_WALNUT: f32 = INFINITY;

const HEALING_HP_HUMAN: f32 = 30.0;
const HEALING_HP_BEAR: f32 = 60.0;
const HEALING_HP_FOX: f32 = 10.0;
const HEALING_HP_WALNUT: f32 = 5.0;

const WEAK_HP_RATIO: f32 = 0.9;
/// 自作の Plugin に Plugin トレイトを実装すれば、Plugin として使用できる
/// Plugin トレイトでは App Builder に必要な要素を追加するだけで良い
impl Plugin for HpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StomachTimer(Timer::new(
            std::time::Duration::from_millis(100),
            true,
        )))
        .add_system(stomach_timer)
        .add_system(hungry)
        .add_system_set(
            SystemSet::new()
                .label("eat")
                .with_system(eat_walnut)
                .with_system(eat_fox)
                .with_system(eat_bear)
                .with_system(eat_human),
        )
        .add_system_set(
            SystemSet::new()
                .after("eat")
                .label("eaten")
                .with_system(eaten_walnut)
                .with_system(eaten_fox)
                .with_system(eaten_bear)
                .with_system(eaten_human),
        );
    }
}

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
fn stomach_timer(time: Res<Time>, mut timer: ResMut<StomachTimer>) {
    timer.0.tick(time.delta());
}

fn hungry(
    mut commands: Commands,
    timer: ResMut<StomachTimer>,
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
    mut walnut_query: Query<(Entity, &Position, &mut HP), (With<Walnut>, Without<WalnutEater>)>,
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
    walnut_query: Query<(Entity, &Position, &HP), With<Walnut>>,
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        walnut_query.iter().for_each(|(_, w_pos, _)| {
            if e_pos == w_pos {
                e_hp.val += HEALING_HP_WALNUT;
            }
        })
    })
}

fn eaten_fox(
    eater_query: Query<(Entity, &Position, &HP), With<FoxEater>>,
    mut fox_query: Query<(Entity, &Position, &mut HP), (With<Fox>, Without<FoxEater>)>,
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
    fox_query: Query<(Entity, &Position, &HP), With<Fox>>,
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        fox_query.iter().for_each(|(_, f_pos, _)| {
            if e_pos == f_pos {
                e_hp.val += HEALING_HP_FOX;
            }
        })
    })
}

fn eaten_bear(
    eater_query: Query<(Entity, &Position, &HP), With<BearEater>>,
    mut bear_query: Query<(Entity, &Position, &mut HP), (With<Bear>, Without<BearEater>)>,
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
    bear_query: Query<(Entity, &Position, &HP), With<Bear>>,
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        bear_query.iter().for_each(|(_, b_pos, b_hp)| {
            if b_pos == e_pos && b_hp.val < b_hp.max * WEAK_HP_RATIO {
                e_hp.val += HEALING_HP_BEAR;
            }
        })
    })
}

fn eaten_human(
    eater_query: Query<(Entity, &Position, &HP), With<HumanEater>>,
    mut human_query: Query<(Entity, &Position, &mut HP), (With<Human>, Without<HumanEater>)>,
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
    human_query: Query<(Entity, &Position, &HP), With<Human>>,
) {
    eater_query.iter_mut().for_each(|(_, e_pos, mut e_hp)| {
        human_query.iter().for_each(|(_, h_pos, h_hp)| {
            if h_pos == e_pos && e_hp.val > e_hp.max * WEAK_HP_RATIO {
                e_hp.val += HEALING_HP_HUMAN;
            }
        })
    })
}
