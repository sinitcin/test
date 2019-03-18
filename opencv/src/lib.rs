///
/// Нужно продемонстрировать умение работы с FFI.
///
/// Для этого возьмём библиотеку OpenCV и простой пример на Си.
///
/// ```C
/// #include <cv.h>
/// #include <highgui.h>
/// #include <stdlib.h>
/// #include <stdio.h>
///
/// IplImage* image = 0;
///
/// int main(int argc, char* argv[])
/// {
///         //! имя картинки задаётся первым параметром
///         char* filename = argc >= 2 ? argv[1] : "Image0.jpg";
///         //! получаем картинку
///         image = cvLoadImage(filename,1);
///
///         printf("[i] image: %s\n", filename);
///         assert( image != 0 );
///
///         cvNamedWindow("origianl", CV_WINDOW_AUTOSIZE);
///         cvNamedWindow("ROI", CV_WINDOW_AUTOSIZE);  
///
///         //! задаём ROI
///         int x = argc >= 3 ? atoi(argv[2]) : 40;
///         int y = argc >= 4 ? atoi(argv[3]) : 20;
///         int width = argc >= 5 ? atoi(argv[4]) : 100;
///         int height = argc >= 6 ? atoi(argv[5]) : 100;
///         //! добавочная величина
///         int add =  argc >= 7 ? atoi(argv[6]) : 200;
///
///         cvShowImage( "origianl", image);
///         //! устанавливаем ROI
///         cvSetImageROI(image, cvRect(x,y,width,height));
///         cvAddS(image, cvScalar(add), image);
///         //! сбрасываем ROI
///         cvResetImageROI(image);
///         //! показываем изображение
///         cvShowImage("ROI", image);
///
///         //! ждём нажатия клавиши
///         cvWaitKey(0);
///
///         //! освобождаем ресурсы
///         cvReleaseImage( &image );
///         cvDestroyAllWindows();
///         return 0;
/// }
/// ```
extern crate libc;
extern crate sharedlib;
use libc::{c_char, c_int, c_void};
use sharedlib::Func;
use sharedlib::Lib;
use sharedlib::Symbol;
use std::ffi::CString;

const _CV_LOAD_IMAGE_UNCHANGED: i32 = -1;
const _CV_LOAD_IMAGE_GRAYSCALE: i32 = 0;
const _CV_LOAD_IMAGE_COLOR: i32 = 1;
const _CV_LOAD_IMAGE_ANYDEPTH: i32 = 2;
const _CV_LOAD_IMAGE_ANYCOLOR: i32 = 4;
const _CV_LOAD_IMAGE_IGNORE_ORIENTATION: i32 = 128;

const _CV_WINDOW_NORMAL: i32 = 0x00000000;
const _CV_WINDOW_AUTOSIZE: i32 = 0x00000001;
const _CV_WINDOW_OPENGL: i32 = 0x00001000;

#[repr(C)]
pub struct CvRect {
    _x: i32,
    _y: i32,
    w: i32,
    h: i32,
}

#[repr(C)]
pub struct CvScalar {
    d0: f64,
    d1: f64,
    d2: f64,
    d3: f64,
}

fn load_lib() -> sharedlib::Result<Lib> {
    let lib = unsafe {
        let path_to_lib = "opencv_world401.dll";
        Lib::new(path_to_lib)
    };
    lib
}

pub fn cv_load_image(filename: &str, iscolor: c_int) -> sharedlib::Result<c_void> {
    Ok(unsafe {
        let lib = load_lib()?;
        let func: Func<extern "C" fn(filename: *const c_char, iscolor: c_int) -> c_void> =
            lib.find_func("cvLoadImage")?;
        let func = func.get();
        let c_str_fname = CString::new(filename).unwrap();
        let c_ptr: *const c_char = c_str_fname.as_ptr();
        func(c_ptr, iscolor)
    })
}

pub fn cv_named_window(name: &str, flags: c_int) -> sharedlib::Result<c_int> {
    Ok(unsafe {
        let lib = load_lib()?;
        let func: Func<extern "C" fn(name: *const c_char, flags: c_int) -> c_int> =
            lib.find_func("cvNamedWindow")?;
        let func = func.get();
        let c_str_name = CString::new(name).unwrap();
        let c_ptr: *const c_char = c_str_name.as_ptr();
        func(c_ptr, flags)
    })
}
/*
#[cfg(windows)]
#[link(name = "opencv_world401.dll", kind = "raw-dylib")]
extern {
    // IplImage* cvLoadImage (const char *filename, int iscolor=CV_LOAD_IMAGE_COLOR)
    // fn cvLoadImage(filename: &c_char, iscolor: i32) -> i32;
    // int cvNamedWindow(char * name, int flags = CV_WINDOW_AUTOSIZE)
    fn cvNamedWindow(name: &c_char, flags: i32) -> i32;
    // void cvShowImage(const char * name, const CvArr * image)
    fn cvShowImage(name: &c_char, image: *const u32);
    // void cvSetImageROI(IplImage * image, CvRect rect)
    fn cvSetImageROI(image: *const u32, rect: CvRect);
    // void cvAddS(const CvArr* src, CvScalar value, CvArr* dst, const CvArr* mask = NULL)
    fn cvAddS(src: *const u32, value: CvScalar, dst: *const u32, mask: *const u32);
    // void cvResetImageROI(IplImage *  image)
    fn cvResetImageROI(image: *const u32);
    // int cvWaitKey(int delay = 0)
    fn cvWaitKey(delay: i32) -> i32;
    // void cvReleaseImage(IplImage** image)
    fn cvReleaseImage(image: *const u32);
    // void cvDestroyAllWindows( void )
    fn cvDestroyAllWindows();
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opencv_test() {
        let _lib = super::load_lib();
        let _image = cv_load_image("C:\\1.jpeg", 1).unwrap();
        let _window = cv_named_window("origianl", _CV_WINDOW_AUTOSIZE).unwrap();
        let _window = cv_named_window("ROI", _CV_WINDOW_AUTOSIZE).unwrap();
    }
}
