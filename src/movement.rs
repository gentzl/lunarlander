use std::f32::consts::PI;

use crate::{lunarmodule, useractions};
use macroquad::prelude::*;

const GRAVITY: f32 = 1.2; // in pixel

const THRUST_CHANGE_PER_FRAME: f32 = 0.10;
const RORATION_CHANGE_PER_FRAME: f32 = 5.0;
const FUEL_CHANGE_PER_FRAME: f32 = 0.4;

pub fn move_lunar_module(
    lunar_module: &mut lunarmodule::LunarModule,
    game_audio: &mut crate::gameaudio::GameAudio,
    user_actions: &useractions::UserAction,
) {
    lunar_module.thrust_active = false;

    if user_actions.thrust_active() && lunar_module.fuel > 0.0 {
        game_audio.exhaust();
        lunar_module.thrust += THRUST_CHANGE_PER_FRAME;
        lunar_module.thrust_active = true;
        lunar_module.fuel -= FUEL_CHANGE_PER_FRAME;

        if lunar_module.fuel < 0.0 {
            lunar_module.fuel = 0.0;
        }
    } else {
        lunar_module.thrust -= THRUST_CHANGE_PER_FRAME / 2.0;
    }

    if lunar_module.thrust < 0.0 {
        lunar_module.thrust = 0.0;
    }

    if user_actions.rotate_right() {
        lunar_module.rotation += RORATION_CHANGE_PER_FRAME;
        if lunar_module.rotation >= 360.0 {
            lunar_module.rotation = lunar_module.rotation - 360.0;
        }
    } else if user_actions.rotate_left() {
        lunar_module.rotation -= RORATION_CHANGE_PER_FRAME;
        if lunar_module.rotation < 0.0 {
            lunar_module.rotation = 360.0 + lunar_module.rotation;
        }
    }

    let new_position = calculate_positions(lunar_module);
    lunar_module.position.x = new_position.x;
    lunar_module.position.y = new_position.y;

    let debug_text = format!(
        "x: {:.2}, y: {:.2}, thrust: {:.2}, rotation: {:.2}",
        lunar_module.position.x, lunar_module.position.y, lunar_module.thrust, lunar_module.rotation
    );

    draw_text(&debug_text, 850.0, 15.0, 15.0, RED);
    // println!("lunar_module: {:?}", lunar_module);
}

pub fn calculate_positions(lunar_module: &mut lunarmodule::LunarModule) -> Vec2 {
    let new_relative_y = (PI / 180.0 * lunar_module.rotation).cos() * lunar_module.thrust;
    let new_relative_x = (PI / 180.0 * lunar_module.rotation).sin() * lunar_module.thrust;
    let current_relative_position = Vec2::new(new_relative_x, new_relative_y - GRAVITY);
    lunar_module.current_relative_position = current_relative_position;

    Vec2::new(
        lunar_module.position.x + current_relative_position.x,
        lunar_module.position.y - current_relative_position.y,
    )
}
