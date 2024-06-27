use std::collections::HashMap;

pub struct LearningState {
    pub q: HashMap<String, (f32, f32, f32)>,
    pub old_state_key: String,
}

impl LearningState {
    pub fn new() -> Self {
        LearningState {
            q: HashMap::new(),
            old_state_key: "".to_string(),
        }
    }
}
