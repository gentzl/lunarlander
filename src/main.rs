use std::{default, mem::transmute, thread, time::Duration};

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
    let use_q_learning = true;
    let user_actions = &mut UserAction::new();
    let learning_state = &mut learning_state::LearningState::new();

    let mut game_audio = gameaudio::GameAudio::new();
    let mut coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
    // println!("{:?}", coordinates);
    let mut lunar_module = lunarmodule::create_initial_lunar_module();
    let mut gamestate = GameState::NotLanded;
    loop {
        if use_q_learning {
            learn(
                learning_state,
                user_actions,
                &mut gamestate,
                lunar_module,
                &mut coordinates,
            );
        }

        if gamestate != GameState::NotLanded {
            show_game_over(&gamestate, &mut game_audio);

            if user_actions.restart() {
                // restart
                gamestate = GameState::NotLanded;
                game_audio.reset();
                coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
                lunar_module = lunarmodule::create_initial_lunar_module();
            }

            next_frame().await;
            continue;
        }

        clear_background(BLACK);
        draw_text("LUNAR LANDER", 20.0, 20.0, 30.0, DARKGRAY);

        movement::move_lunar_module(&mut lunar_module, &mut game_audio, &user_actions);
        map::draw(&coordinates);
        lunarmodule::draw(lunar_module).await;
        fuel::draw(lunar_module.fuel);
        gamestate = gamestate::calculate(lunar_module, &coordinates);

        if gamestate != GameState::NotLanded {
            continue;
        }

        let frame_time = get_frame_time();
        if frame_time < MINIMUM_TIME_FRAME {
            let time_to_sleep = (MINIMUM_TIME_FRAME - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
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
