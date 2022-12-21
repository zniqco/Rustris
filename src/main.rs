#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;
mod rustris;

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    rustris::init();

    loop {
        rustris::update();
        rustris::draw();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Rustris"),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        sample_count: 0,
        high_dpi: true,
        icon: None,
        ..Default::default()
    }
}
