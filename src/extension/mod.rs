use macroquad::prelude::*;
use macroquad::miniquad::*;

pub fn get_additive_material() -> Material {
    static mut MATERIAL: Option<Material> = None;

    unsafe {
        if let None = MATERIAL {
            let material = load_material(
                include_str!("default.vert"),
                include_str!("default.frag"),
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

            MATERIAL = Some(material);
        }

        return MATERIAL.unwrap();
    }
}

pub fn draw_text_aligned(text: &str, x: f32, y: f32, font: Font, font_size: u16, pivot_x: f32, pivot_y: f32, color: Color) {
    let font_scale = 1.0;
    let dimensions = measure_text(text, Some(font), font_size, font_scale);

    draw_text_ex(text, x - dimensions.width * pivot_x, y + dimensions.offset_y - dimensions.height * pivot_y, TextParams {
        font,
        font_size,
        font_scale,
        color,
        ..Default::default()
    });
}

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
}