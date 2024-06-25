use crate::{gameaudio, lunarmodule, map::SurfaceCoordinate, MAX_WINDOW_HEIGHT, MAX_WINDOW_WIDTH};
pub const IMAGE_CORRECTION_Y: f32 = 10.0;

use macroquad::prelude::*;
#[derive(PartialEq, Debug)]
pub enum GameState {
    NotLanded,
    Landed,
    Crashed,
}

pub fn calculate(
    lunar_module: lunarmodule::LunarModule,
    coordinates: &Vec<SurfaceCoordinate>,
) -> GameState {
    let lunar_module_y_corrected = lunar_module.position.y + IMAGE_CORRECTION_Y;

    // draw_circle(lunar_module.position.x, lunar_module_y_corrected, 3.0, RED);
    let nearest_coordinate_right_option =
        coordinates.iter().find(|c| c.x >= lunar_module.position.x);

    if nearest_coordinate_right_option.is_none() {
        return GameState::NotLanded;
    }

    let nearest_coordinate_right = nearest_coordinate_right_option.unwrap();
    let index = coordinates
        .iter()
        .position(|c| c.x == nearest_coordinate_right.x)
        .unwrap();
    let nearest_coordinate_left_option = coordinates.get(index - 1);

    if nearest_coordinate_left_option.is_none() {
        return GameState::NotLanded;
    }
    let nearest_coordinate_left = nearest_coordinate_left_option.unwrap();
    let mut game_state = check_crashed(
        nearest_coordinate_left,
        nearest_coordinate_right,
        lunar_module.position.x,
        lunar_module_y_corrected,
    );

    if game_state == GameState::Crashed {
        return game_state;
    }

    game_state = check_landed(
        nearest_coordinate_left,
        lunar_module_y_corrected,
        lunar_module.rotation,
        lunar_module.current_relative_position.y,
    );
    game_state
}

fn check_landed(
    nearest_coordinate_left: &SurfaceCoordinate,
    y: f32,
    rotation: f32,
    current_relative_y: f32,
) -> GameState {
    if nearest_coordinate_left.is_landing_zone_left && y >= nearest_coordinate_left.y {
        if current_relative_y < 1.5 && (rotation < 8.0 || rotation > 352.0) {
            return GameState::Landed;
        }
    }
    GameState::NotLanded
}

fn check_crashed(
    nearest_coordinate_left: &SurfaceCoordinate,
    nearest_coordinate_right: &SurfaceCoordinate,
    x: f32,
    y: f32,
) -> GameState {
    let m = (nearest_coordinate_right.y - nearest_coordinate_left.y)
        / (nearest_coordinate_right.x - nearest_coordinate_left.x);
    let y_surface = m * (x - nearest_coordinate_left.x) + nearest_coordinate_left.y;

    if y - 10.0 > y_surface {
        return GameState::Crashed;
    }
    GameState::NotLanded
}

pub fn show_game_over(gamestate: &GameState, game_audio: &mut gameaudio::GameAudio) {
    match gamestate {
        GameState::Landed => {
            draw_text(
                "LANDED",
                MAX_WINDOW_WIDTH / 2.0 - 100.0,
                MAX_WINDOW_HEIGHT / 2.0,
                50.0,
                GREEN,
            );
            game_audio.won();
        }
        GameState::Crashed => {
            draw_text(
                "CRASHED",
                MAX_WINDOW_WIDTH / 2.0 - 100.0,
                MAX_WINDOW_HEIGHT / 2.0,
                50.0,
                RED,
            );
            game_audio.lost();
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
