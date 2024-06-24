use macroquad::prelude::*;

pub fn draw(fuel: f32) {
    // max fuel
    draw_rectangle(screen_width() / 2.0 - 100.0, 5.0, 100.0 * 3.0, 10.0, GRAY);
    // current fuel
    let color = match fuel as i32 {
        0..=20 => RED,
        21..=50 => YELLOW,
        _ => GREEN,
    };
    draw_rectangle(screen_width() / 2.0 - 100.0, 5.0, fuel * 3.0, 10.0, color);
}
