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



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
