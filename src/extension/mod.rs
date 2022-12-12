use macroquad::prelude::*;

pub fn draw_text_aligned(text: &str, x: f32, y: f32, font: Font, font_size: u16, pivot_x: f32, pivot_y: f32, color: Color) {
    let font_scale = 1.0;
    let dimensions = measure_text(text, Some(font), font_size, font_scale);

    draw_text_ex(text, x - dimensions.width * pivot_x, y - dimensions.height * pivot_y, TextParams {
        font,
        font_size,
        font_scale,
        color,
        ..Default::default()
    });
}