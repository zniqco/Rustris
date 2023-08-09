use std::collections::HashMap;
use lazy_static::lazy_static;
use macroquad::audio::Sound;
use macroquad::prelude::*;
use macroquad::miniquad::*;
use futures::executor;

macro_rules! include_textures {
    ($n:ident $f:ident $e:literal) => {{
        let bytes = include_bytes!(concat!("../../include/", $e, ".", stringify!($f)));
        let texture = Texture2D::from_file_with_format(bytes, None);

        $n.insert($e.to_string(), texture);
    }};
    ($n:ident $f:ident $e:literal $($nn:ident $ff:ident $ee:literal)*) => {
        include_textures! { $n $f $e }
        include_textures! { $($nn $ff $ee)* }
    };
}

macro_rules! include_sounds {
    ($n:ident $f:ident $e:literal) => {{
        let bytes = include_bytes!(concat!("../../include/", $e, ".", stringify!($f)));
        let sound = executor::block_on(macroquad::audio::load_sound_from_bytes(bytes)).unwrap();

        $n.insert($e.to_string(), sound);
    }};
    ($n:ident $f:ident $e:literal $($nn:ident $ff:ident $ee:literal)*) => {
        include_sounds! { $n $f $e }
        include_sounds! { $($nn $ff $ee)* }
    };
}

lazy_static! {
    pub static ref TEXTURES: HashMap<String, Texture2D> = {
        let mut map = HashMap::new();

        include_textures! {
            map png "blocks"
            map jpg "background_1"
            map jpg "background_2"
            map jpg "background_3"
            map jpg "background_4"
        }

        map
    };
    pub static ref SOUNDS: HashMap<String, Sound> = {
        let mut map = HashMap::new();

        include_sounds! {
            map ogg "ready"
            map ogg "go"
            map ogg "move"
            map ogg "rotate"
            map ogg "rotate_spin"
            map ogg "hard_drop"
            map ogg "lock"
            map ogg "hold"
            map ogg "erase"
            map ogg "erase_quad"
            map ogg "tspin"
            map ogg "level_up"
            map ogg "game_over"
        }

        map
    };
    pub static ref FONT_DEFAULT: Font = load_ttf_font_from_bytes(include_bytes!("../../include/font.otf")).unwrap();
    pub static ref MATERIAL_ADDITIVE: Material = make_material_additive();
}

fn make_material_additive() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("../../include/default.vert"),
            fragment: include_str!("../../include/default.frag"),
        },
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
    ).unwrap()
}

pub fn init_resources() {
    lazy_static::initialize(&TEXTURES);
    lazy_static::initialize(&SOUNDS);
    lazy_static::initialize(&FONT_DEFAULT);
    lazy_static::initialize(&MATERIAL_ADDITIVE);
}

pub fn texture(key: &str) -> &Texture2D {
    &TEXTURES[key]
}

pub fn sound(key: &str) -> &Sound {
    &SOUNDS[key]
}
