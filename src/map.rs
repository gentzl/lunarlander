use ::rand::{self as random_crate, Rng};
use macroquad::prelude::*;

const STEPS_WIDTH: f32 = 30.0;
const MAX_HEIGTH_CHANGE: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct SurfaceCoordinate {
    pub x: f32,
    pub y: f32,
    pub is_landing_zone_left: bool,
    pub is_landing_zone_right: bool,
}

pub fn generate_coordinates(max_width: f32, max_height: f32) -> Vec<SurfaceCoordinate> {
    let min_height = max_height - 10.0;
    let steps_width = (max_width / STEPS_WIDTH) as i32;
    let max_mountain_heigth = max_height / 2.5;

    let landing_zone_step = random_crate::thread_rng().gen_range(2..STEPS_WIDTH as i32 - 2);

    let mut coordinates: Vec<SurfaceCoordinate> = Vec::new();

    for n in 0..steps_width + 1 {
        let last_coordinate = coordinates.last();
        let last_y = match last_coordinate {
            Some(c) => c.y,
            None => max_height - MAX_HEIGTH_CHANGE,
        };

        let x = n as f32 * STEPS_WIDTH;
        let mut y: f32;

        let random_y_change: f32 =
            random_crate::thread_rng().gen_range(-MAX_HEIGTH_CHANGE..MAX_HEIGTH_CHANGE);

        if landing_zone_step == n {
            y = last_y;
            let coordinate = SurfaceCoordinate {
                x,
                y,
                is_landing_zone_left: false,
                is_landing_zone_right: true,
            };
            coordinates.push(coordinate);
            continue;
        }

        // relative changes to the last y
        y = last_y + random_y_change;
        if y < max_mountain_heigth {
            y = last_y + MAX_HEIGTH_CHANGE;
        }

        if y > min_height {
            y = min_height;
        }

        let is_landing_zone_left = if landing_zone_step - 1 == n {
            true
        } else {
            false
        };

        let coordinate = SurfaceCoordinate {
            x,
            y,
            is_landing_zone_left: is_landing_zone_left,
            is_landing_zone_right: false,
        };

        coordinates.push(coordinate);
    }

    coordinates
}

pub fn draw(coordinates: &Vec<SurfaceCoordinate>) {
    for n in 1..coordinates.len() {
        let coordinate = &coordinates[n];
        let previous_coordinate = &coordinates[n - 1];

        let color = if coordinate.is_landing_zone_right {
            GREEN
        } else {
            DARKGRAY
        };

        draw_line(
            previous_coordinate.x,
            previous_coordinate.y,
            coordinate.x,
            coordinate.y,
            5.0,
            color,
        );
    }
}
