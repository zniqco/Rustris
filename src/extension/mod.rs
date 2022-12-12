use macroquad::prelude::*;
use macroquad::miniquad::*;

pub fn get_additive_material() -> Material {
    static mut MATERIAL: Option<Material> = None;

    const DEFAULT_VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    color = color0 / 255.0;
}"#;

    const DEFAULT_FRAGMENT: &str = r#"#version 100
varying lowp vec2 uv;
varying lowp vec4 color;
uniform sampler2D Texture;
void main() {
    gl_FragColor = color * texture2D(Texture, uv);
}"#;

    unsafe {
        if let None = MATERIAL {
            let material = load_material(
                DEFAULT_VERTEX,
                DEFAULT_FRAGMENT,
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

    draw_text_ex(text, x - dimensions.width * pivot_x, y - dimensions.height * pivot_y, TextParams {
        font,
        font_size,
        font_scale,
        color,
        ..Default::default()
    });
}
