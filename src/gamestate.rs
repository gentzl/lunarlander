use std::f32::consts::PI;

use crate::{lunarmodule, map::SurfaceCoordinate};
use macroquad::prelude::*;
#[derive(PartialEq, Debug)]
pub enum GameState {
    NotLanded,
    Landed,
    Crashed,
}

// consider the image height
const LUNAR_LANDER_Y_SIZE_BUFFER: f32 = 30.0;

pub fn calculate(
    lunar_module: lunarmodule::LunarModule,
    coordinates: &Vec<SurfaceCoordinate>,
) -> GameState {
    let lunar_module_y_corrected = lunar_module.position.y + LUNAR_LANDER_Y_SIZE_BUFFER;
    let nearest_coordinate_left = coordinates.iter().find(|c| c.x >= lunar_module.position.x);
    let nearest_coordinate_right = coordinates.iter().find(|c| c.x <= lunar_module.position.x);
    // println!("nearest_coordinate: {:?}", nearest_coordinate);
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
        lunar_module.trust,
    );
    game_state
}

fn check_landed(
    nearest_coordinate_left: Option<&SurfaceCoordinate>,
    y: f32,
    rotation: f32,
    trust: f32,
) -> GameState {
    match nearest_coordinate_left {
        Some(c) => {
            if c.is_landing_zone_left && y >= c.y {
                if trust < 16.0 && (rotation < 8.0 || rotation > 352.0) {
                    return GameState::Landed;
                }
            }
            GameState::NotLanded
        }

        None => GameState::NotLanded,
    }
}

fn check_crashed(
    nearest_coordinate_left_option: Option<&SurfaceCoordinate>,
    nearest_coordinate_right_option: Option<&SurfaceCoordinate>,
    x: f32,
    y: f32,
) -> GameState {
    match nearest_coordinate_left_option {
        Some(c) => {
            if nearest_coordinate_right_option.is_none() {
                return GameState::NotLanded;
            }

            let nearest_coordinate_right = nearest_coordinate_right_option.unwrap();
            let m = (nearest_coordinate_right.y - c.y) / (nearest_coordinate_right.x - c.x);
            let y_surface = m * (x - c.x) + c.y;

            if y - 10.0 > y_surface {
                return GameState::Crashed;
            }
            GameState::NotLanded
        }

        None => GameState::NotLanded,
    }
}
