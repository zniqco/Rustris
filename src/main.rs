//#![windows_subsystem = "windows"]
mod rustris;

use rustris::Rustris;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rustris")
        .build();

    let mut rustris = Rustris::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    rustris.init(&mut rl, &thread);

    while !rl.window_should_close() {
        rustris.update(&mut rl);
        rustris.draw(&mut rl.begin_drawing(&thread));
    }
}
