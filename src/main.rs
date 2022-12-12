#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod rustris;
mod game;
mod extension;

use macroquad::prelude::*;
use rustris::Rustris;

#[macroquad::main(window_conf)]
async fn main() {
    let mut rustris = Rustris::new();

    loop {
        rustris.update();
        rustris.draw();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rustris"),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        sample_count: 1,
        high_dpi: true,
        icon: None,
        ..Default::default()
    }
}
