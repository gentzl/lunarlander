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
    epsilon: f32, // exploration rate
) {
    let landing_zone_left = coordinates.iter().find(|c| c.is_landing_zone_left).unwrap();

    let reward_alive: f32 = 1.0;
    let reward_nearer: f32 = 1.5;
    let reward_nearer_x: f32 = 5.0;
    let reward_nearer_y: f32 = 0.1;
    let reward_rotation_in_range = 3.0;
    let reward_crashed = -1000.0;
    let reward_landed = 10000.00;
    let alpha: f32 = 0.2; // learning rate
    let gamma: f32 = 0.98; // discount factor

    learning_state.counter += 1;

    if game_state == &GameState::Landed {
        learning_state.win_loose.0 += 1;
        //println!("!!!!!!!!!!!!!!Landed")
    } else if game_state == &GameState::Crashed {
        learning_state.win_loose.1 += 1;
    }
    // left,right, trust, none
    let mut q_value = (0.5, 0.5, 0.5, 0.5);

    let relative_x = landing_zone_left.x - lunar_module.position.x;
    let relative_y = landing_zone_left.y - lunar_module.position.y;
    let current_relative_position = Vec2::new(relative_x, relative_y);
    let state_key = build_key(lunar_module, current_relative_position, landing_zone_left);

    if !learning_state.q.contains_key(&state_key) {
        learning_state.q.insert(state_key.clone(), q_value);
        learning_state.current_new_states_updated += 1;
    } else {
        q_value = *learning_state.q.get(&state_key).unwrap();
        learning_state.current_old_states_load_updated += 1;
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
            GameState::Landed => {
                // print position and relative position
                /* println!(
                    "position: {:?}, relative_position: {:?}",
                    lunar_module.position, current_relative_position
                ); */
                reward_landed
            }
            _ => reward_alive,
        };

        /*

               if (lunar_module.trust_active) {
                   reward -= 0.2;
               }
        */
        /*
               // if relative x==0 reward
               if relative_x.abs() < 10.0 {
                   reward += reward_nearer_x * 2.0;
               }

               if learning_state.old_relative_position.is_some() {
                   if current_relative_position.y.abs()
                       < learning_state.old_relative_position.unwrap().y.abs()
                   {
                       reward -= reward_nearer_y;

                       // reward for the x position
                       // println!("reward_nearer_y: {}", reward_nearer_y);
                   }
               }
        */
        /*   // reward if trust > 2
                if is_far_away(
                    lunar_module.current_relative_position.x as i32,
                    lunar_module.current_relative_position.y as i32,
                ) && lunar_module.trust > 3.0
                    && lunar_module.trust < 8.0
                {
                    reward += 3.0;
                }
        */
        /*
        if is_far_away(
            current_relative_position.x as i32,
            current_relative_position.y as i32,
        ) && current_relative_position.y.abs() > 250.0
        {
            reward += relative_y
        }*/
        if current_relative_position.x.abs() == 0.0
            && is_far_away(
                current_relative_position.x as i32,
                current_relative_position.y as i32,
            )
            && lunar_module.position.y > 350.0
        {
            reward -= 100.0;
        }

        if relative_y < 0.0 {
            reward -= 100.0;
        }

        if current_relative_position.x.abs() == 0.0 && current_relative_position.y.abs() < 100.0 {
            reward += 25.0;
        }

        //  println!("current_relative_position: {:?}", current_relative_position);
        // reward if relative position is near to the landing zone
        if learning_state.old_relative_position.is_some() {
            if current_relative_position.x.abs()
                < learning_state.old_relative_position.unwrap().x.abs()
            {
                reward += reward_nearer;

                // reward for the x position
                //   println!("reward_nearer_x: {}", reward_nearer_x);
            }
        }

        //println!("current_relative_position: {:?}", current_relative_position);
        /*
        if current_relative_position.x.abs() == 0.0
            && (lunar_module.rotation <= 8.0 || lunar_module.rotation >= 352.0)
        {
            reward += reward_rotation_in_range;
            if current_relative_position.y.abs() < 60.0 {
                if relative_y.abs() < 2.0 {
                    reward += reward_nearer * 2.0;
                }
            }
        }
         */

        if current_relative_position.x.abs() == 0.0 && current_relative_position.y.abs() == 0.0 {
            reward += 500.0;
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

        learning_state.current_reward += reward;
    }
    user_actions.set_action(UserActionSimulation::None);

    if q_value.0 >= q_value.1 && q_value.0 >= q_value.2 && q_value.0 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::RotateLeft);
    } else if q_value.1 >= q_value.0 && q_value.1 >= q_value.2 && q_value.1 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::RotateRight);
    } else if q_value.2 >= q_value.0 && q_value.2 >= q_value.1 && q_value.2 >= q_value.3 {
        user_actions.set_action(UserActionSimulation::TrustActive);
    }

    // do random action if epsilon is greater than random value
    if epsilon > rand::random::<f32>() {
        let random_action = rand::random::<f32>();
        if random_action < epsilon {
            let randomAction: UserActionSimulation = rand::random();
            //print!("random action: {:?}", randomAction);
            user_actions.set_action(randomAction);
        }
    }

    learning_state.old_state_key = state_key.clone();
    learning_state.old_relative_position = Some(current_relative_position);

    if game_state != &GameState::NotLanded {
        user_actions.set_action(UserActionSimulation::Restart);
        learning_state.old_state_key = "".to_string();
    }
}

fn build_key(
    lunar_module: LunarModule,
    current_relative_position: Vec2,
    landing_zone: &SurfaceCoordinate,
) -> String {
    let mut relative_x_key = round_10(current_relative_position.x);
    let mut relative_y_key = round_10(current_relative_position.y);
    let mut rotation_key = round_rotation(lunar_module.rotation);
    let mut current_relative_y_key = round_relative(lunar_module.current_relative_position.y);

    let is_far_away = is_far_away(relative_x_key, relative_y_key);
    let mut trust = lunar_module.trust as i32;
    if trust > 5 {
        trust = 100;
    } else if trust < -4 {
        trust = -100;
    }

    let fuel: i32 = round_fuel(lunar_module.fuel as i32);

    if is_far_away {
        relative_x_key = round_200(current_relative_position.x);
        relative_y_key = round_200(current_relative_position.y);
    }
    format!(
        "_{},{},_{}_{}_{}_{}",
        relative_x_key, relative_y_key, rotation_key, current_relative_y_key, trust, fuel
    )
}

fn is_far_away(relative_x_key: i32, relative_y_key: i32) -> bool {
    relative_x_key.abs() >= 250 || relative_y_key.abs() >= 250
}

fn round_200(value: f32) -> i32 {
    ((value / 35.0).round() * 35.0) as i32
}

fn round_10(value: f32) -> i32 {
    ((value / 10.0).round() * 10.0) as i32
}

fn round_30(value: f32) -> i32 {
    ((value / 30.0).round() * 30.0) as i32
}

fn round_rotation(value: f32) -> i32 {
    if value <= 45.0 || value > 315.0 {
        return ((value / 8.0).round() * 8.0) as i32;
    }

    return ((value / 45.0).round() * 45.0) as i32;
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
