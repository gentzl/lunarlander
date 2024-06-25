use std::f32::consts::PI;

use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct LunarModule {
    pub rotation: f32,
    pub position: Vec2,
    pub current_relative_position: Vec2,
    pub trust: f32,
    pub trust_active: bool,
    pub fuel: f32,
}

pub const ROCKET_IMAGE_SIZE: f32 = 45.0;

const SHIP_STARTING_ROTATION: f32 = 0.0;
const SHIP_STARTING_TRUST: f32 = 0.0;
const SHIP_STARTING_X: f32 = 150.;
const SHIP_STARTING_Y: f32 = 150.;

pub async fn draw(lunarmodule: LunarModule) {
    let path = if lunarmodule.trust_active {
        "src/images/rocket32.png"
    } else {
        "src/images/rocket32_no_trust.png"
    };

    let corrected_x = lunarmodule.position.x - ROCKET_IMAGE_SIZE / 2.0;
    let corrected_y = lunarmodule.position.y - ROCKET_IMAGE_SIZE / 2.0;
    let texture: Texture2D = load_texture(path).await.unwrap();
    draw_texture_ex(
        &texture,
        corrected_x,
        corrected_y,
        WHITE,
        DrawTextureParams {
            rotation: lunarmodule.rotation * PI / 180.0,
            ..Default::default()
        },
    );
}

pub fn create_initial_lunar_module() -> LunarModule {
    LunarModule {
        rotation: SHIP_STARTING_ROTATION,
        position: Vec2 {
            x: SHIP_STARTING_X,
            y: SHIP_STARTING_Y,
        },
        current_relative_position: Vec2 { x: 0.0, y: 0.0 },
        trust: SHIP_STARTING_TRUST,
        trust_active: false,
        fuel: 100.0,
    }
}
