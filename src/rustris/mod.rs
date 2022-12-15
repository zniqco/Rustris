mod functions;
mod resources;
mod scenes;

use macroquad::prelude::*;
use macroquad::color::colors;
use functions::*;
use resources::*;
use scenes::*;

pub enum Scene {
    None,
    Menu,
    Ingame,
}

enum InternalScene {
    None,
    Menu(Menu),
    Ingame(Ingame),
}

static mut current_scene: InternalScene = InternalScene::None;

pub fn init() {
    init_resources();
    set_scene(Scene::Menu);
}

pub fn update() {
    unsafe {
        match &mut current_scene {
            InternalScene::Menu(x) => x.update(),
            InternalScene::Ingame(x) => x.update(),
            _ => { },
        }
    }
}

pub fn draw() {
    clear_background(colors::BLACK);

    draw_texture_ex(*BACKGROUND_TEXTURE, 0.0, 0.0, colors::WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(screen_width(), screen_height())),
        ..Default::default()
    });

    unsafe {
        match &current_scene {
            InternalScene::Menu(x) => x.draw(),
            InternalScene::Ingame(x) => x.draw(),
            _ => { },
        }
    }

    draw_text_aligned(get_fps().to_string().as_str(), screen_width() - 12.0, 12.0, *DEFAULT_FONT, 14, 1.0, 0.0, colors::WHITE);
}

pub fn set_scene(scene: Scene) {
    unsafe {
        match scene {
            Scene::None => current_scene = InternalScene::None,
            Scene::Menu => current_scene = InternalScene::Menu(Menu::new()),
            Scene::Ingame => current_scene = InternalScene::Ingame(Ingame::new()),
        }
    }
}
