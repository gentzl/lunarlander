

use gamestate::GameState::{self};
use macroquad::prelude::*;
mod fuel;
mod gamestate;
mod gamestate_test;
mod lunarmodule;
mod map;
mod movement;
mod movement_test;
mod test;

const MAX_WINDOW_WIDTH: f32 = 1200.;
const MAX_WINDOW_HEIGHT: f32 = 700.;
const MINIMUM_TIME_FRAME: f32 = 1. / 15.; // 15 frames per second

#[macroquad::main(window_conf)]
async fn main() {
    let coordinates = map::generate_coordinates(MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT);
    // println!("{:?}", coordinates);
    let mut lunar_module = lunarmodule::create_initial_lunar_module();
    let mut gamestate = GameState::NotLanded;
    loop {
        if gamestate != GameState::NotLanded {
            show_game_over(&gamestate);

            if is_key_down(KeyCode::Enter) {
                gamestate = GameState::NotLanded;
                lunar_module = lunarmodule::create_initial_lunar_module();
            }

            next_frame().await;
            continue;
        }
        clear_background(BLACK);
        draw_text("LUNAR LANDER", 20.0, 20.0, 30.0, DARKGRAY);
        let frame_time = get_frame_time();
        movement::move_lunar_module(&mut lunar_module);
        map::draw(&coordinates);
        lunarmodule::draw(lunar_module).await;
        fuel::draw(lunar_module.fuel);
        gamestate = gamestate::calculate(lunar_module, &coordinates);

        if gamestate != GameState::NotLanded {
            continue;
        }

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

fn show_game_over(gamestate: &GameState) {
    match gamestate {
        GameState::Landed => {
            draw_text(
                "LANDED",
                MAX_WINDOW_WIDTH / 2.0 - 100.0,
                MAX_WINDOW_HEIGHT / 2.0,
                50.0,
                GREEN,
            );
        }
        GameState::Crashed => {
            draw_text(
                "CRASHED",
                MAX_WINDOW_WIDTH / 2.0 - 100.0,
                MAX_WINDOW_HEIGHT / 2.0,
                50.0,
                RED,
            );
        }
        _ => {}
    }
    draw_text(
        "Hit 'Enter' to restart",
        MAX_WINDOW_WIDTH / 2.0 - 80.0,
        MAX_WINDOW_HEIGHT / 2.0 + 15.0,
        15.0,
        GRAY,
    );
}
