use macroquad::math::Vec2;

use crate::{
    gamestate::GameState,
    learning_state::LearningState,
    lunarmodule::LunarModule,
    map::SurfaceCoordinate,
    useractions::{UserAction, UserActionSimulation},
};

const REWARD_ALIVE: f32 = 0.1;
const REWARD_NEARER: f32 = 4.5;
const REWARD_X0_Y0: f32 = 1700.0;
const REWARD_CRASHED: f32 = -5000.0;
const REWARD_LANDED: f32 = 10000.00;
const ALPHA: f32 = 0.2; // learning rate
const GAMMA: f32 = 0.99; // discount factor

pub fn learn(
    learning_state: &mut LearningState,
    user_actions: &mut UserAction,
    game_state: &mut GameState,
    lunar_module: LunarModule,
    coordinates: &mut Vec<SurfaceCoordinate>,
    epsilon: f32, // exploration rate
) {
    let landing_zone_left = coordinates.iter().find(|c| c.is_landing_zone_left).unwrap();

    learning_state.counter += 1;
    learning_state.current_counter += 1;

    if game_state == &GameState::Landed {
        learning_state.win_loose.0 += 1;
        learning_state.current_win_loose.0 += 1;
        learning_state.current_consumed_fuel += round_fuel(lunar_module.fuel as i32);
    } else if game_state == &GameState::Crashed {
        learning_state.win_loose.1 += 1;
        learning_state.current_win_loose.1 += 1;
    }
    // left,right, trust, none (default probability)
    let mut q_value = (0.5, 0.5, 0.5, 0.5);

    let relative_x = landing_zone_left.x - lunar_module.position.x;
    let relative_y = landing_zone_left.y - lunar_module.position.y;
    let current_relative_position = Vec2::new(relative_x, relative_y);
    let state_key = build_key(lunar_module, current_relative_position);

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
            GameState::Crashed => REWARD_CRASHED,
            GameState::Landed => REWARD_LANDED + 0.5 * (REWARD_LANDED / 100.0 * lunar_module.fuel), // reward if fuel is left
            _ => REWARD_ALIVE,
        };

        if current_relative_position.x.abs() == 0.0 && current_relative_position.y.abs() < 150.0 {
            reward += 30.0;
        }

        // reward if relative position is near to the landing zone
        if learning_state.old_relative_position.is_some() {
            if current_relative_position.x.abs()
                < learning_state.old_relative_position.unwrap().0.abs()
            {
                reward += REWARD_NEARER * 0.2;
                let x_change = learning_state.old_relative_position.unwrap().0.abs()
                    - current_relative_position.x.abs();
                if x_change > 2.7
                    && is_far_away(
                        round_relative(current_relative_position.x as f32),
                        round_relative(current_relative_position.y as f32),
                    )
                {
                    reward += REWARD_NEARER * x_change;
                }
            }
        }

        if current_relative_position.x.abs() == 0.0 && current_relative_position.y.abs() == 0.0 {
            reward += REWARD_X0_Y0;
        }

        step(&user_actions.action, old_q_value, reward, q_value_max);

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
            let random_action: UserActionSimulation = rand::random();
            user_actions.set_action(random_action);
        }
    }

    learning_state.old_state_key = state_key.clone();
    learning_state.old_relative_position =
        Some((current_relative_position.x, current_relative_position.y));

    if game_state != &GameState::NotLanded {
        user_actions.set_action(UserActionSimulation::Restart);
        learning_state.old_state_key = "".to_string();
    }
}

fn build_key(lunar_module: LunarModule, current_relative_position: Vec2) -> String {
    let mut relative_x_key = round_5(current_relative_position.x);
    let mut relative_y_key = round_5(current_relative_position.y);
    let rotation_key = round_rotation(lunar_module.rotation);

    let is_far_away = is_far_away(relative_x_key, relative_y_key);
    let mut trust = lunar_module.trust as i32;
    if trust > 6 {
        trust = 100;
    } else if trust < -4 {
        trust = -100;
    }

    if is_far_away {
        relative_x_key = round_35(current_relative_position.x);
        relative_y_key = round_35(current_relative_position.y);
    }
    format!(
        "_{},{},_{}_{}",
        relative_x_key, relative_y_key, rotation_key, trust
    )
}

fn step(
    action: &UserActionSimulation,
    old_q_value: &mut (f32, f32, f32, f32),
    reward: f32,
    q_value_max: f32,
) {
    match action {
        UserActionSimulation::RotateLeft => {
            old_q_value.0 =
                (1.0 - ALPHA) * (old_q_value.0) + ALPHA * (reward + GAMMA * q_value_max);
        }
        UserActionSimulation::RotateRight => {
            old_q_value.1 =
                (1.0 - ALPHA) * (old_q_value.1) + ALPHA * (reward + GAMMA * q_value_max);
        }
        UserActionSimulation::TrustActive => {
            old_q_value.2 =
                (1.0 - ALPHA) * (old_q_value.2) + ALPHA * (reward + GAMMA * q_value_max);
        }
        // e.g. do nothing
        _ => {
            old_q_value.3 =
                (1.0 - ALPHA) * (old_q_value.3) + ALPHA * (reward + GAMMA * q_value_max);
        }
    }
}

fn is_far_away(relative_x_key: i32, relative_y_key: i32) -> bool {
    relative_x_key.abs() >= 250 || relative_y_key.abs() >= 250
}

fn round_35(value: f32) -> i32 {
    ((value / 45.0).round() * 45.0) as i32
}

fn round_5(value: f32) -> i32 {
    ((value / 5.0).round() * 5.0) as i32
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
