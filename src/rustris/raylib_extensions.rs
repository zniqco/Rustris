use std::ffi::CString;
use raylib::prelude::*;

pub trait RaylibDrawHandleExtension {
    fn draw_text_right(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>);
}

impl RaylibDrawHandleExtension for RaylibDrawHandle<'_> {
    fn draw_text_right(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: impl Into<ffi::Color>) {
        let width = measure_text(text, font_size);

        self.draw_text(text, x - width, y, font_size, color);
    }
}

pub trait RaylibHandleExtension {
    fn load_texture_from_bytes(&mut self, thread: &RaylibThread, filetype: &str, bytes: &'static [u8]) -> Option<Texture2D>;
}

impl RaylibHandleExtension for RaylibHandle {
    fn load_texture_from_bytes(&mut self, thread: &RaylibThread, filetype: &str, bytes: &'static [u8]) -> Option<Texture2D> {
        let c_filetype = CString::new(filetype).unwrap();
        let c_bytes = bytes.as_ptr();
        let size = bytes.len() as i32;
        let ffi_image = unsafe { ffi::LoadImageFromMemory(c_filetype.as_ptr(), c_bytes, size) };

        if !ffi_image.data.is_null() {
            return match self.load_texture_from_image(thread, unsafe { &Image::from_raw(ffi_image) }) {
                Ok(tex) => Some(tex),
                Err(_) => None,
            }
        }
        
        None
    }
}