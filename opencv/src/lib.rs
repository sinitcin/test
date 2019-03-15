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
///         // имя картинки задаётся первым параметром
///         char* filename = argc >= 2 ? argv[1] : "Image0.jpg";
///         // получаем картинку
///         image = cvLoadImage(filename,1);
///
///         printf("[i] image: %s\n", filename);
///         assert( image != 0 );
///
///         cvNamedWindow("origianl", CV_WINDOW_AUTOSIZE);
///         cvNamedWindow("ROI", CV_WINDOW_AUTOSIZE);  
///
///         // задаём ROI
///         int x = argc >= 3 ? atoi(argv[2]) : 40;
///         int y = argc >= 4 ? atoi(argv[3]) : 20;
///         int width = argc >= 5 ? atoi(argv[4]) : 100;
///         int height = argc >= 6 ? atoi(argv[5]) : 100;
///         // добавочная величина
///         int add =  argc >= 7 ? atoi(argv[6]) : 200;
///
///         cvShowImage( "origianl", image);
///         // устанавливаем ROI
///         cvSetImageROI(image, cvRect(x,y,width,height));
///         cvAddS(image, cvScalar(add), image);
///         // сбрасываем ROI
///         cvResetImageROI(image);
///         // показываем изображение
///         cvShowImage("ROI", image);
///
///         // ждём нажатия клавиши
///         cvWaitKey(0);
///
///         // освобождаем ресурсы
///         cvReleaseImage( &image );
///         cvDestroyAllWindows();
///         return 0;
/// }
/// ```
#[allow(non_snake_case)]
extern crate libc;

use libc::{c_char, c_int, c_void, size_t};
use std::ffi::{CStr, CString};

const CV_LOAD_IMAGE_UNCHANGED: i32 = -1;
const CV_LOAD_IMAGE_GRAYSCALE: i32 = 0;
const CV_LOAD_IMAGE_COLOR: i32 = 1;
const CV_LOAD_IMAGE_ANYDEPTH: i32 = 2;
const CV_LOAD_IMAGE_ANYCOLOR: i32 = 4;
const CV_LOAD_IMAGE_IGNORE_ORIENTATION: i32 = 128;

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

#[link(name = "opencv_ffmpeg401.dll")]
extern {
    // IplImage* cvLoadImage (const char *filename, int iscolor=CV_LOAD_IMAGE_COLOR)
    fn cvLoadImage(filename: &c_char, iscolor: i32) -> i32;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
