use std::sync::Mutex;
use lazy_static::lazy_static;
use macroquad::{prelude::*, audio::Sound};
use crate::{game::*, resources::*};

lazy_static! {
    static ref QUITTED: Mutex<bool> = Mutex::new(false);
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

pub fn calc_block_position(board_x: f32, board_y: f32, x: i32, y: i32, cell_size: f32) -> (f32, f32) {
    (board_x + x as f32 * cell_size, board_y - (y + 1) as f32 * cell_size)
}

pub fn draw_block(texture: &Texture2D, x: f32, y: f32, cell_size: f32, block_type: BlockType, alpha: f32) {
    match block_type {
        BlockType::Empty | BlockType::Outside => { },
        _ => {
            draw_texture_ex(*texture, x, y, Color::new(1.0, 1.0, 1.0, alpha), DrawTextureParams {
                dest_size: Some(Vec2::new(cell_size, cell_size)),
                source: Some(Rect::new(block_type as i32 as f32 * 30.0, 0.0, 30.0, 30.0)),
                ..Default::default()
            });
        }
    }
}

pub fn draw_preview(texture: &Texture2D, x: f32, y: f32, cell_size: f32, rotation: &RotationType, piece: PieceType, alpha: f32) {
    let piece_data = rotation.blocks(piece);
    let size = (piece_data.shape[0][0].len() as i32, piece_data.shape[0].len() as i32);
    let offset = (size.0 as f32 * -0.5 + piece_data.preview_offset.0, size.1 as f32 * -0.5 + piece_data.preview_offset.1);

    for j in 0..size.1 as usize {
        for i in 0..size.0 as usize {
            if piece_data.shape[0][j][i] == 1 {
                draw_block(texture, x + (i as f32 + offset.0) * cell_size, y + (j as f32 + offset.1) * cell_size, cell_size, piece_data.block, alpha);
            }
        }
    }
}

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32) {
    draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 3.0, Color::from_rgba(255, 255, 255, 255));
    draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.875));
}

pub fn texture(key: &str) -> Texture2D {
    let texture = *(TEXTURES.read().unwrap().get(key).unwrap());

    texture
}

pub fn sound(key: &str) -> Sound {
    let sound = *(SOUNDS.read().unwrap().get(key).unwrap());

    sound
}

pub fn font_default() -> Font {
    *FONT_DEFAULT
}

pub fn material_additive() -> Material {
    *MATERIAL_ADDITIVE
}

pub fn push_matrix_trs(x: f32, y: f32, deg: f32, scale_x: f32, scale_y: f32) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(scale_x, scale_y, 0.0),
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, deg / 180.0 * std::f32::consts::PI),
            Vec3::new(x, y, 1.0)
        ));
    }
}

pub fn pop_matrix() {
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}

pub fn quit() {
    *QUITTED.lock().unwrap() = true;
}

pub fn quitted() -> bool {
    *QUITTED.lock().unwrap()
}

pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
	if a == b {
		0.0
    } else {
	    clamp((v - a) / (b - a), 0.0, 1.0)
    }
}
