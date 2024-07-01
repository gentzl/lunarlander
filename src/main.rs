use serde_json::{Result, Value};
use std::{collections::HashMap, default, fs, mem::transmute, thread, time::Duration};

use gamestate::{show_game_over, GameState};
use macroquad::prelude::*;
use qlearning::learn;
use useractions::UserAction;

mod QLearning;
mod fuel;
mod gameaudio;
mod gamestate;
mod gamestate_test;
mod learning_state;
mod lunarmodule;
mod map;
mod movement;
mod movement_test;
mod qlearning;
mod test;
mod useractions;
const MAX_WINDOW_WIDTH: f32 = 1200.;
const MAX_WINDOW_HEIGHT: f32 = 700.;
const MINIMUM_TIME_FRAME: f32 = 1. / 15.; // 15 frames per second

#[macroquad::main(window_conf)]
async fn main() {
    let use_q_learning: bool = true;
    let show_game = false;
    let user_actions: &mut UserAction = &mut UserAction::new();
    let learning_state = &mut learning_state::LearningState::new();
    load_state(learning_state);

    let mut game_audio = gameaudio::GameAudio::new();
    game_audio.active = !use_q_learning;
    let mut coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
    // println!("{:?}", coordinates);
    let mut lunar_module = lunarmodule::create_initial_lunar_module();
    let mut game_state = GameState::NotLanded;
    let mut counter = 0;
    let mut win_loose = (0, 0);

    loop {
        counter += 1;
        if game_state == GameState::Landed {
            win_loose.0 += 1;
            //println!("!!!!!!!!!!!!!!Landed")
        } else if game_state == GameState::Crashed {
            win_loose.1 += 1;
        }
        if counter % 1000000 == 0 {
            write_state(&learning_state, counter, win_loose);
        }
        if use_q_learning {
            learn(
                learning_state,
                user_actions,
                &mut game_state,
                lunar_module,
                &mut coordinates,
            );
        }

        if game_state != GameState::NotLanded {
            show_game_over(&game_state, &mut game_audio);

            if user_actions.restart() {
                // restart
                game_state = GameState::NotLanded;
                game_audio.reset();
                coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
                lunar_module = lunarmodule::create_initial_lunar_module();
            }
            if show_game {
                next_frame().await;
            }
            continue;
        }
        if show_game {
            clear_background(BLACK);
            draw_text("LUNAR LANDER", 20.0, 20.0, 30.0, DARKGRAY);
        }
        movement::move_lunar_module(&mut lunar_module, &mut game_audio, &user_actions);
        if show_game {
            map::draw(&coordinates);

            lunarmodule::draw(lunar_module).await;
            fuel::draw(lunar_module.fuel);
        }
        game_state = gamestate::calculate(lunar_module, &coordinates);

        if game_state != GameState::NotLanded {
            continue;
        }

        let frame_time = get_frame_time();
        if !use_q_learning && frame_time < MINIMUM_TIME_FRAME {
            let time_to_sleep = (MINIMUM_TIME_FRAME - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        if show_game {
            next_frame().await
        }
    }
}

fn load_state(learning_state: &mut learning_state::LearningState) {
    let file_name_latest = format!("learnings/learn_latest.json");

    if !std::path::Path::new(&file_name_latest).exists() {
        return;
    }

    let serialized = fs::read_to_string(file_name_latest).expect("Unable to read file");
    let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    let q: HashMap<String, (f32, f32, f32, f32)> = serde_json::from_value(deserialized).unwrap();
    learning_state.q = q;

    println!("loaded {} states", learning_state.q.len());
}
fn write_state(
    learning_state: &learning_state::LearningState,
    counter: i32,
    win_loose: (i32, i32),
) {
    let file_name_archive = format!("learnings/learn_{}.json", counter);
    let file_name_latest = format!("learnings/learn_latest.json");
    let path = std::path::Path::new(&file_name_archive);
    let dir = path.parent().unwrap();
    std::fs::create_dir_all(dir).unwrap();
    let serialized = serde_json::to_string(&learning_state.q).unwrap();

    fs::write(file_name_archive, serialized.clone()).expect("Unable to write file");
    fs::write(file_name_latest, serialized).expect("Unable to write file");
    let mut win_rate: f32 = 0.0;
    if win_loose.0 > 0 {
        let count = win_loose.0 + win_loose.1;
        win_rate = win_loose.0 as f32 / count as f32;
    }
    println!(
        "write_state: {}, win_loose_rate:{} ({},{})",
        counter, win_rate, win_loose.0, win_loose.1
    );
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: MAX_WINDOW_WIDTH as i32,
        window_height: MAX_WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}
