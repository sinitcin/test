extern crate libc;
use libc::{c_char, c_int, c_void};

type VipsImage = u32;

#[link(name = "vips")]
extern "C" {
    pub fn vips_image_new_from_file(filename: *const c_char) -> VipsImage;
}

#[cfg(test)]
mod tests {

use std::ffi::CString;


    #[test]
    fn it_works() {
        unsafe {            
            let image = super::vips_image_new_from_file(CString::new("/home/anton/Изображения/1587607.jpg").expect("CString::new failed").as_ptr());            
            assert_ne!(image as u32, 0, "Ошибка загрузки изображения!");

            let image = super::vips_image_new_from_file(CString::new("1.jpg").expect("CString::new failed").as_ptr());            
            assert_ne!(image as u32, 0, "Ошибка загрузки изображения!");
        }        
    }
}
