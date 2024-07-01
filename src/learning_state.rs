use std::collections::HashMap;

use macroquad::math::Vec2;

pub struct LearningState {
    pub q: HashMap<String, (f32, f32, f32, f32)>,
    pub old_state_key: String,
    pub old_relative_position: Option<Vec2>,
}

impl LearningState {
    pub fn new() -> Self {
        LearningState {
            q: HashMap::new(),
            old_state_key: "".to_string(),
            old_relative_position: None,
        }
    }
}
