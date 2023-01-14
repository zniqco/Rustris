use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use macroquad::audio::Sound;
use macroquad::prelude::*;
use macroquad::miniquad::*;

macro_rules! include_resources {
    (sound $e:literal.$f:ident) => {{
        let bytes = include_bytes!(concat!("../include/", $e, ".", stringify!($f)));
        let sound = macroquad::audio::load_sound_from_bytes(bytes).await.unwrap();
    
        SOUNDS.write().unwrap().insert($e.to_string(), sound);
    }};
    (texture $e:literal.$f:ident) => {{
        let bytes = include_bytes!(concat!("../include/", $e, ".", stringify!($f)));
        let texture = Texture2D::from_file_with_format(bytes, None);

        TEXTURES.write().unwrap().insert($e.to_string(), texture);
    }};
    ($n:ident $e:literal.$f:ident $($nn:ident $ee:literal.$ff:ident)*) => {
        include_resources! { $n $e.$f }
        include_resources! { $($nn $ee.$ff)* }
    };
}

lazy_static! {
    pub static ref TEXTURES: RwLock<HashMap<String, Texture2D>> = RwLock::new(HashMap::new());
    pub static ref SOUNDS: RwLock<HashMap<String, Sound>> = RwLock::new(HashMap::new());
    pub static ref FONT_DEFAULT: Font = load_ttf_font_from_bytes(include_bytes!("../include/font.otf")).unwrap();
    pub static ref MATERIAL_ADDITIVE: Material = load_material(
        include_str!("../include/default.vert"),
        include_str!("../include/default.frag"),
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::One
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    ).unwrap();
}

pub async fn init_resources() {
    clear_background(Color::new(0.0, 0.0, 0.0, 1.0));

    let text = "Loading...";
    let text_center = get_text_center(text, Some(*FONT_DEFAULT), 24, 1.0, 0.0);

    draw_text_ex(text, screen_width() * 0.5 - text_center.x, screen_height() * 0.5 + text_center.y, TextParams {
        font: *FONT_DEFAULT,
        font_size: 24,
        color: Color::new(0.5, 0.5, 0.5, 1.0),
        ..Default::default()
    });

    next_frame().await;

    lazy_static::initialize(&FONT_DEFAULT);
    lazy_static::initialize(&MATERIAL_ADDITIVE);

    include_resources! {
        texture "blocks".png
        texture "background_1".jpg
        texture "background_2".jpg
        texture "background_3".jpg
        texture "background_4".jpg
        sound "ready".ogg
        sound "go".ogg
        sound "move".ogg
        sound "rotate".ogg
        sound "rotate_spin".ogg
        sound "hard_drop".ogg
        sound "lock".ogg
        sound "hold".ogg
        sound "erase".ogg
        sound "erase_quad".ogg
        sound "tspin".ogg
        sound "level_up".ogg
        sound "game_over".ogg
    }

    next_frame().await; // HACK: Hack for initialize delay
    next_frame().await;
}
