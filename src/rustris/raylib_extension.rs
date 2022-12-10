use raylib::prelude::*;

pub trait RaylibExtension {
    fn draw_text_right(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>);
}

impl RaylibExtension for RaylibDrawHandle<'_> {
    fn draw_text_right(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) {
        let width = measure_text(text, font_size);

        self.draw_text(text, x - width, y, font_size, color);
    }
}