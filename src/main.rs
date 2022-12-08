//#![windows_subsystem = "windows"]
mod rustris;

use rustris::Rustris;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Rustris")
        .build();

    let mut rustris = Rustris::new();

    rustris.init();

    while !rl.window_should_close() {
        rustris.update(&mut rl);
        rustris.draw(&mut rl.begin_drawing(&thread));
    }
}
