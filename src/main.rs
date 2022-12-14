#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;
mod rustris;
mod utility;

use macroquad::prelude::*;
use rustris::*;

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
        window_resizable: true,
        sample_count: 1,
        high_dpi: true,
        icon: None,
        ..Default::default()
    }
}
