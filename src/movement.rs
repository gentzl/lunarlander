use std::f32::consts::PI;

use crate::lunarmodule;
use macroquad::prelude::*;

const GRAVITY: f32 = 2.0; // in pixel
const TRUST_CHANGE_PER_FRAME: f32 = 0.25;
const RORATION_CHANGE_PER_FRAME: f32 = 5.0;
const FUEL_CHANGE_PER_FRAME: f32 = 0.7;

pub fn move_lunar_module(lunar_module: &mut lunarmodule::LunarModule) {
    lunar_module.trust_active = false;

    if is_key_down(KeyCode::Up) && lunar_module.fuel > 0.0 {
        lunar_module.trust += TRUST_CHANGE_PER_FRAME;
        lunar_module.trust_active = true;
        lunar_module.fuel -= FUEL_CHANGE_PER_FRAME;

        if lunar_module.fuel < 0.0 {
            lunar_module.fuel = 0.0;
        }
    } else if is_key_down(KeyCode::Down) {
        lunar_module.trust -= TRUST_CHANGE_PER_FRAME / 2.0;
    } else {
        lunar_module.trust -= TRUST_CHANGE_PER_FRAME / 2.0;
    }

    if lunar_module.trust < 0.0 {
        lunar_module.trust = 0.0;
    }

    if is_key_down(KeyCode::Right) {
        lunar_module.rotation += RORATION_CHANGE_PER_FRAME;
        if lunar_module.rotation >= 360.0 {
            lunar_module.rotation = lunar_module.rotation - 360.0;
        }
    } else if is_key_down(KeyCode::Left) {
        lunar_module.rotation -= RORATION_CHANGE_PER_FRAME;
        if lunar_module.rotation < 0.0 {
            lunar_module.rotation = 360.0 + lunar_module.rotation;
        }
    }

    let new_position = calculate_position(&lunar_module);
    lunar_module.position.x = new_position.x;
    lunar_module.position.y = new_position.y;

    let debug_text = format!(
        "x: {:.2}, y: {:.2}, trust: {:.2}, rotation: {:.2}",
        lunar_module.position.x, lunar_module.position.y, lunar_module.trust, lunar_module.rotation
    );

    draw_text(&debug_text, 850.0, 15.0, 15.0, RED);
    // println!("lunar_module: {:?}", lunar_module);
}

pub fn calculate_position(lunar_module: &lunarmodule::LunarModule) -> Vec2 {
    let new_relative_y = (PI / 180.0 * lunar_module.rotation).cos() * lunar_module.trust;
    let new_relative_x = (PI / 180.0 * lunar_module.rotation).sin() * lunar_module.trust;

    Vec2::new(
        lunar_module.position.x + new_relative_x,
        lunar_module.position.y - new_relative_y + GRAVITY,
    )
}
