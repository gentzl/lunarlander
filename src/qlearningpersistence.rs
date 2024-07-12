use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::learning_state;

pub fn load_state() -> learning_state::LearningState {
    let file_name_latest = format!("learnings/learn_latest.json");

    if !std::path::Path::new(&file_name_latest).exists() {
        return learning_state::LearningState::new();
    }

    let serialized = fs::read_to_string(file_name_latest).expect("Unable to read file");
    let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    let learning_state: learning_state::LearningState =
        serde_json::from_value(deserialized).unwrap();

    println!("loaded {} states", learning_state.q.len());
    learning_state
}

pub fn write_state(learning_state: &learning_state::LearningState) {
    let file_name_archive = format!("learnings/learn_{}.json", learning_state.counter);
    let file_name_latest = format!("learnings/learn_latest.json");
    let path = std::path::Path::new(&file_name_archive);
    let dir = path.parent().unwrap();
    std::fs::create_dir_all(dir).unwrap();
    let serialized = serde_json::to_string(&learning_state).unwrap();
    fs::write(file_name_archive, serialized.clone()).expect("Unable to write file");
    fs::write(file_name_latest, serialized).expect("Unable to write file");
    let mut win_rate: f32 = 0.0;
    if learning_state.win_loose.0 > 0 {
        let count = learning_state.win_loose.0 + learning_state.win_loose.1;
        win_rate = learning_state.win_loose.0 as f32 / count as f32;
    }
    let mut current_win_rate: f32 = 0.0;
    if learning_state.current_win_loose.0 > 0 {
        let count = learning_state.current_win_loose.0 + learning_state.current_win_loose.1;
        current_win_rate = learning_state.current_win_loose.0 as f32 / count as f32;
    }
    println!(
        "write_state: {}, win_loose_rate (overall):{} ({},{}),
        win_loose_rate (current):{} ({},{}), reward: {}, new_states: {}, old_states: {}, games_played: {}",
        learning_state.counter,
        win_rate,
        learning_state.win_loose.0,
        learning_state.win_loose.1,
        current_win_rate,
        learning_state.current_win_loose.0,
        learning_state.current_win_loose.1,
        learning_state.current_reward,
        learning_state.current_new_states_updated,
        learning_state.current_old_states_load_updated,
        learning_state.games_played
    );
}
