use lazy_static::lazy_static;
use macroquad::prelude::*;
use macroquad::miniquad::*;

lazy_static! {
    pub static ref BACKGROUND_TEXTURE: Texture2D = Texture2D::from_file_with_format(include_bytes!("../../assets/background.png"), None);

    pub static ref BLOCKS_TEXTURE: Texture2D = Texture2D::from_file_with_format(include_bytes!("../../assets/blocks.png"), None);

    pub static ref DEFAULT_FONT: Font = load_ttf_font_from_bytes(include_bytes!("../../assets/font.ttf")).unwrap();

    pub static ref ADDITIVE_MATERIAL: Material = load_material(
        include_str!("../../assets/default.vert"),
        include_str!("../../assets/default.frag"),
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

pub fn init_resources() {
    lazy_static::initialize(&BACKGROUND_TEXTURE);
    lazy_static::initialize(&BLOCKS_TEXTURE);
    lazy_static::initialize(&DEFAULT_FONT);
    lazy_static::initialize(&ADDITIVE_MATERIAL);
}
