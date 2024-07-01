use std::{collections::HashMap, f32::consts::FRAC_1_SQRT_2, thread, time::Duration};

use macroquad::{math::Vec2, miniquad::gl::GL_PROGRAM_POINT_SIZE};
use rand::distributions::OpenClosed01;

use crate::{
    gamestate::GameState,
    learning_state::LearningState,
    lunarmodule::{self, LunarModule},
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

    let reward_alive: f32 = 0.002;
    let reward_nearer: f32 = 0.002;
    let reward_rotation_in_range = 0.1;
    let reward_crashed = -5.00;
    let reward_landed = 100.00;
    let alpha: f32 = 0.2; // learning rate
    let gamma: f32 = 0.9; // discount factor

    // left,right, trust, none
    let mut q_value = (1.0, 1.0, 1.0, 1.0);

    let relative_x = lunar_module.position.x - landing_zone_left.x;
    let relative_y = lunar_module.position.y - landing_zone_left.y;
    let current_relative_position = Vec2::new(relative_x, relative_y);
    let state_key = build_key(lunar_module, current_relative_position);

    if !learning_state.q.contains_key(&state_key) {
        learning_state.q.insert(state_key.clone(), q_value);
    } else {
        q_value = *learning_state.q.get(&state_key).unwrap();
    }

    // the best way to go
    let q_value_max = get_max(q_value, game_state);

    // recalculate q depend on the game state before
    if !learning_state.old_state_key.is_empty() {
        let old_q_value = learning_state
            .q
            .get_mut(learning_state.old_state_key.as_str())
            .unwrap();

        let mut reward = match game_state {
            GameState::Crashed => reward_crashed,
            GameState::Landed => reward_landed,
            _ => 0.0,
        };

        //  println!("current_relative_position: {:?}", current_relative_position);
        // reward if relative position is near to the landing zone
        if learning_state.old_relative_position.is_some() {
            if current_relative_position.x.abs()
                < learning_state.old_relative_position.unwrap().x.abs()
            {
                reward += reward_nearer;
            }
        }

        // current_relative_position
        // println!("current_relative_position: {:?}", current_relative_position);
        /*if !is_far_away(
            current_relative_position.x as i32,
            current_relative_position.y as i32,
        ) && (lunar_module.rotation <= 8.0 || lunar_module.rotation >= 352.0)
        {
            reward += reward_rotation_in_range;
        }*/
        if relative_x == 0.0 {
            reward += reward_nearer;
        }

        if !is_far_away(
            current_relative_position.x as i32,
            current_relative_position.y as i32,
        ) && (lunar_module.rotation <= 25.0 || lunar_module.rotation >= 335.0)
        {
            reward += reward_rotation_in_range / 4.0;
        }

        // reward_alive
        if game_state == &GameState::NotLanded {
            reward += reward_alive;
        }

        match user_actions.action {
            UserActionSimulation::RotateLeft => {
                old_q_value.0 =
                    (1.0 - alpha) * (old_q_value.0) + alpha * (reward + gamma * q_value_max);
            }
            UserActionSimulation::RotateRight => {
                old_q_value.1 =
                    (1.0 - alpha) * (old_q_value.1) + alpha * (reward + gamma * q_value_max);
            }
            UserActionSimulation::TrustActive => {
                old_q_value.2 =
                    (1.0 - alpha) * (old_q_value.2) + alpha * (reward + gamma * q_value_max);
            }
            // e.g. do nothing
            _ => {
                old_q_value.3 =
                    (1.0 - alpha) * (old_q_value.3) + alpha * (reward + gamma * q_value_max);
            }
        }
    }
    user_actions.set_action(UserActionSimulation::None);

    if q_value.0 >= q_value.1 && q_value.0 >= q_value.2 && q_value.0 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::RotateLeft);
    } else if q_value.1 >= q_value.0 && q_value.1 >= q_value.2 && q_value.1 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::RotateRight);
    } else if q_value.2 >= q_value.0 && q_value.2 >= q_value.1 && q_value.2 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::TrustActive);
    }

    learning_state.old_state_key = state_key.clone();
    learning_state.old_relative_position = Some(current_relative_position);

    if game_state != &GameState::NotLanded {
        user_actions.set_action(UserActionSimulation::Restart);
        learning_state.old_state_key = "".to_string();
    }
}

fn build_key(lunar_module: LunarModule, current_relative_position: Vec2) -> String {
    let mut relative_x_key = round_10(current_relative_position.x);
    let mut relative_y_key = round_10(current_relative_position.y);
    let mut rotation_key = round_rotation(lunar_module.rotation);
    let mut current_relative_y_key = round_relative(lunar_module.current_relative_position.y);

    let is_far_away = is_far_away(relative_x_key, relative_y_key);
    let mut trust = lunar_module.trust as i32;
    if trust > 2 {
        trust = 100;
    } else if trust < 0 {
        trust = -1;
    }

    let fuel = round_fuel(lunar_module.fuel as i32);

    if is_far_away {
        relative_x_key = round_200(current_relative_position.x);
        relative_y_key = round_200(current_relative_position.y);
        rotation_key = 1;
        current_relative_y_key = 1;
    }
    format!(
        "_{},{},_{}_{}_{}_{}",
        relative_x_key, relative_y_key, rotation_key, current_relative_y_key, trust, fuel
    )
}

fn is_far_away(relative_x_key: i32, relative_y_key: i32) -> bool {
    relative_x_key.abs() >= 100 || relative_y_key.abs() >= 100
}

fn round_200(value: f32) -> i32 {
    ((value / 100.0).round() * 100.0) as i32
}

fn round_10(value: f32) -> i32 {
    ((value / 10.0).round() * 10.0) as i32
}

fn round_rotation(value: f32) -> i32 {
    if value <= 45.0 || value > 315.0 {
        return ((value / 8.0).round() * 8.0) as i32;
    }

    return ((value / 90.0).round() * 90.0) as i32;
}

fn round_fuel(value: i32) -> i32 {
    return (value / 30) * 30 as i32;
}

fn round_relative(value: f32) -> i32 {
    value.round() as i32
}

fn get_max(q_value: (f32, f32, f32, f32), game_state: &mut GameState) -> f32 {
    if game_state == &GameState::Crashed || game_state == &GameState::Landed {
        return 0.0;
    }

    if q_value.0 > q_value.1 && q_value.0 > q_value.2 && q_value.0 > q_value.3 {
        return q_value.0;
    } else if q_value.1 > q_value.0 && q_value.1 > q_value.2 && q_value.1 > q_value.3 {
        return q_value.1;
    } else if q_value.2 > q_value.0 && q_value.2 > q_value.1 && q_value.2 > q_value.3 {
        return q_value.2;
    }
    q_value.3
}
