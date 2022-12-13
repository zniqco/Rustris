use std::f32::consts::PI;
use macroquad::prelude::*;
use super::*;

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

pub fn get_block_position(board_x: f32, board_y: f32, x: i32, y: i32, cell_size: f32) -> (f32, f32) {
    (board_x + x as f32 * cell_size, board_y - (y + 1) as f32 * cell_size)
}

pub fn draw_block(x: f32, y: f32, cell_size: f32, block_type: BlockType, alpha: f32) {
    match block_type {
        BlockType::Empty | BlockType::Outside => { },
        _ => {
            draw_texture_ex(*BLOCKS_TEXTURE, x, y, Color::new(1.0, 1.0, 1.0, alpha), DrawTextureParams {
                dest_size: Some(Vec2::new(cell_size, cell_size)),
                source: Some(Rect::new(block_type as i32 as f32 * 30.0, 0.0, 30.0, 30.0)),
                ..Default::default()
            });
        }
    }
}

pub fn draw_preview(x: f32, y: f32, cell_size: f32, piece_type: PieceType, alpha: f32) {
    let array = match piece_type {
        PieceType::Z => [(-1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (1.0, 0.5)],
        PieceType::S => [(1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (-1.0, 0.5)],
        PieceType::L => [(-1.0, 0.5), (0.0, 0.5), (1.0, 0.5), (1.0, -0.5)],
        PieceType::J => [(-1.0, -0.5), (-1.0, 0.5), (0.0, 0.5), (1.0, 0.5)],
        PieceType::I => [(-1.5, 0.0), (-0.5, 0.0), (0.5, 0.0), (1.5, 0.0)],
        PieceType::O => [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)],
        PieceType::T => [(-1.0, 0.5), (0.0, 0.5), (0.0, -0.5), (1.0, 0.5)],
    };

    for cell in array {
        draw_block(x + ((cell.0 - 0.5) * cell_size), y + ((cell.1 - 0.5) * cell_size), cell_size, piece_type.get_block_type(), alpha);
    }
}

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32) {
    draw_rectangle_lines(x - 2.0, y - 2.0, width + 4.0, height + 4.0, 2.0, colors::WHITE);
    draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.875));
}

pub fn push_matrix_scale(x: f32, y: f32) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(Mat4::from_scale(Vec3::new(x, y, 1.0)));
    }
}

pub fn push_matrix_translation(x: f32, y: f32) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(Mat4::from_translation(Vec3::new(x, y, 0.0)));
    }
}

pub fn push_matrix_rotation(deg: f32) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(Mat4::from_euler(EulerRot::XYZ, 0.0, 0.0, deg / 180.0 * PI));
    }
}

pub fn pop_matrix() {
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}
