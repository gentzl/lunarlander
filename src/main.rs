use serde_json::{Result, Value};
use std::{
    collections::HashMap, default, env, fs, mem::transmute, process, thread, time::Duration,
};

use gamestate::{show_game_over, GameState};
use macroquad::prelude::*;
use qlearning::learn;
use useractions::UserAction;

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
mod qlearningpersistence;
mod test;
mod useractions;
const MAX_WINDOW_WIDTH: f32 = 1200.;
const MAX_WINDOW_HEIGHT: f32 = 700.;
const MINIMUM_TIME_FRAME: f32 = 1. / 30.; // 15 frames per second

#[macroquad::main(window_conf)]
async fn main() {
    let mut use_q_learning: bool = false;
    let mut show_game = true;
    let mut epsilon = 0.0;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-l" | "--learn" => use_q_learning = true,
            "-e" | "--explorerate" => epsilon = args.next().unwrap().parse().unwrap(),

            "-h" | "--hidegame" => {
                show_game = false;
            }

            _ => {
                if arg.starts_with('-') {
                    println!("Unkown argument {}", arg);
                } else {
                    println!("Unkown positional argument {}", arg);
                }
            }
        }
    }

    println!(
        "use_q_learning: {}, epsilon: {}, show_game: {}",
        use_q_learning, epsilon, show_game
    );
    let user_actions: &mut UserAction = &mut UserAction::new();
    let learning_state = &mut qlearningpersistence::load_state();

    let mut game_audio = gameaudio::GameAudio::new();
    game_audio.active = !use_q_learning;
    let mut coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
    let mut lunar_module = lunarmodule::create_initial_lunar_module();
    let mut game_state = GameState::NotLanded;
    learning_state.current_win_loose = (0, 0);
    learning_state.current_reward = 0.0;
    learning_state.current_new_states_updated = 0;
    learning_state.current_old_states_load_updated = 0;
    learning_state.current_counter = 0;
    learning_state.current_consumed_fuel = 0;

    loop {
        if use_q_learning
            && learning_state.current_counter > 0
            && learning_state.current_counter % 1000000 == 0
        {
            qlearningpersistence::write_state(&learning_state, epsilon);
            process::exit(1);
        }
        if use_q_learning {
            learn(
                learning_state,
                user_actions,
                &mut game_state,
                lunar_module,
                &mut coordinates,
                epsilon,
            );
        }

        if game_state != GameState::NotLanded {
            learning_state.games_played += 1;
            if show_game {
                show_game_over(&game_state, &mut game_audio);
            }
            if user_actions.restart() {
                // restart
                game_state = GameState::NotLanded;
                game_audio.reset();
                coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
                lunar_module = lunarmodule::create_initial_lunar_module();
                lunar_module.trust = 2.0;
                // create random start x position for the lunar module after restart
                let start_x = rand::gen_range(200, (MAX_WINDOW_WIDTH - 200.0) as i32) as f32;
                lunar_module.position.x = start_x;
                // random rotation
                lunar_module.rotation = rand::gen_range(0, 360) as f32;
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
        if show_game && frame_time < MINIMUM_TIME_FRAME {
            let time_to_sleep = (MINIMUM_TIME_FRAME - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        if show_game {
            next_frame().await
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: MAX_WINDOW_WIDTH as i32,
        window_height: MAX_WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}
