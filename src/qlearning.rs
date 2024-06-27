use std::{collections::HashMap, thread, time::Duration};

use rand::distributions::OpenClosed01;

use crate::{
    gamestate::GameState,
    learning_state::LearningState,
    lunarmodule::LunarModule,
    map::SurfaceCoordinate,
    useractions::{UserAction, UserActionSimulation},
};

pub fn learn(
    learning_state: &mut LearningState,
    user_actions: &mut UserAction,
    game_state: &mut GameState,
    lunar_module: LunarModule,
    coordinates: &mut Vec<SurfaceCoordinate>,
) {
    let landing_zone_left = coordinates.iter().find(|c| c.is_landing_zone_left).unwrap();

    let reward_alive = 1.0;
    let reward_crashed = -10.000;
    //    let reward_landed = 100.000;
    let alpha: f32 = 0.2; // learning rate
    let gamma: f32 = 0.9; // discount factor

    let mut q_value = (0.0, 0.0, 0.0);
    let state_key = build_key(landing_zone_left, lunar_module);

    if !learning_state.q.contains_key(&state_key) {
        println!("inserted key: {}", state_key);
        learning_state.q.insert(state_key.clone(), q_value.clone());
    } else {
        q_value = *learning_state.q.get(&state_key).unwrap();
    }

    // the best way to go
    let q_value_max = get_max(q_value);

    // recalculate q depend on the game state before
    if !learning_state.old_state_key.is_empty() {
        println!("old_state key: {}", learning_state.old_state_key);
        let old_q_value = learning_state
            .q
            .get_mut(learning_state.old_state_key.as_str())
            .unwrap();

        match user_actions.action {
            UserActionSimulation::RotateLeft => {
                old_q_value.0 =
                    (1.0 - alpha) * (old_q_value.0) + alpha * (reward_alive + gamma * q_value_max);
            }
            UserActionSimulation::TrustActive => {
                old_q_value.1 =
                    (1.0 - alpha) * (old_q_value.1) + alpha * (reward_alive + gamma * q_value_max);
            }
            _ => {
                old_q_value.2 =
                    (1.0 - alpha) * (old_q_value.2) + alpha * (reward_alive + gamma * q_value_max);
            }
        }
    }

    //let old_q_value = q.get(user_actions.old_state_key.as_str()).unwrap();
    user_actions.set_action(UserActionSimulation::None);

    if q_value.0 > q_value.1 && q_value.0 > q_value.2 {
        user_actions.set_action(UserActionSimulation::RotateLeft);
    } else if q_value.1 > q_value.0 && q_value.1 > q_value.2 {
        user_actions.set_action(UserActionSimulation::TrustActive);
    }

    learning_state.old_state_key = state_key.clone();

    if game_state != &GameState::NotLanded {
        user_actions.set_action(UserActionSimulation::Restart);
    }
}

fn build_key(landing_zone_left: &SurfaceCoordinate, lunar_module: LunarModule) -> String {
    format!(
        //"landing_zone_left:{}, {},lunar_module: {}, {},{}",
        "_{},{},_:{},{},{}",
        round(landing_zone_left.x),
        round(landing_zone_left.y),
        round(lunar_module.position.x),
        round(lunar_module.position.y),
        round_relative(lunar_module.current_relative_position.y)
    )
}
fn round(value: f32) -> i32 {
    ((value / 10.0).round() * 10.0) as i32
}

fn round_relative(value: f32) -> i32 {
    value.round() as i32
}

fn get_max(q_value: (f32, f32, f32)) -> f32 {
    if q_value.0 > q_value.1 && q_value.0 > q_value.2 {
        return q_value.0;
    } else if q_value.1 > q_value.0 && q_value.1 > q_value.2 {
        return q_value.1;
    }
    q_value.2
}
