use std::collections::HashMap;

use macroquad::math::Vec2;

pub struct LearningState {
    pub q: HashMap<String, (f32, f32, f32, f32)>,
    pub old_state_key: String,
    pub old_relative_position: Option<Vec2>,
    pub win_loose: (i32, i32),
    pub current_win_loose: (i32, i32),
    pub current_reward: f32,
    pub counter: i64,
    pub current_old_states_load_updated: i32,
    pub current_new_states_updated: i32,
    pub games_played: i32,
}

impl LearningState {
    pub fn new() -> Self {
        LearningState {
            q: HashMap::new(),
            old_state_key: "".to_string(),
            old_relative_position: None,
            win_loose: (0, 0),
            current_win_loose: (0, 0),
            current_reward: 0.0,
            counter: 0,
            current_old_states_load_updated: 0,
            current_new_states_updated: 0,
            games_played: 0,
        }
    }
}
