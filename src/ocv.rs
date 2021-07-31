use std::path::PathBuf;

use image::{ImageBuffer, Rgba};
use opencv::{core::{self, CV_8UC1, CV_8UC3, CV_8UC4, UMatUsageFlags}, highgui::{self, WINDOW_AUTOSIZE}, imgcodecs, imgproc::{self, COLOR_RGBA2BGRA, LINE_8}, prelude::*, stitching, video, videoio};

use opencv::core::AccessFlag::ACCESS_READ;
use winapi::um::wingdi::Ellipse;

use crate::screen::DXGIManager;
pub async fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, WINDOW_AUTOSIZE)?;
    // let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/right.png");
    // let mat = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;

    // let img = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
    loop {
        // let image = get_screen().await.unwrap();
        // let mut frame = Mat::from_gpumat();
        let mut frame = Mat::default();
        let start = Mat::ones( 600, 600, CV_8UC4)?.to_mat()?;
        // start.convert_to(&mut frame, CV_8UC4, 255f64, 0f64)?;
        // imgproc::cvt_color(&start, &frame, code, dst_cn);

        // let res = imgproc::line(&mut frame, core::Point::new(5, 5), core::Point::new(500, 500), core::Scalar::new(50f64, 50f64, 50f64, 50f64), 3, LINE_8, 0 );
		// imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        // 	// cam.read(&mut frame)?;
        // if frame.size()?.width > 0 {
        highgui::imshow(window, &frame)?;
        // }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

async fn get_screen() -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    use imageproc::window::display_image;
    let mut manager = DXGIManager::new(300).unwrap();
    let _ = manager.capture_frame(); // first frame is black all time
    match manager.capture_frame() {
        Ok((pixels, (w, h))) => {
            let len = pixels.len() as u64;
            println!("len: {} w: {} h:{}", len, w, h);

            let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);
            for x in 0..w {
                for y in 0..h {
                    let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                    let index = x + y * w;

                    // println!("r: {:?} g: {:?} b:{:?} a:{:?}", pixels[index].r, pixels[index].g, pixels[index].b, pixels[index].a);
                    *pixel = image::Rgba([
                        pixels[index].r,
                        pixels[index].g,
                        pixels[index].b,
                        pixels[index].a,
                    ]);

                    // *pixel = image::Rgba([200u8, 0u8, 155u8, 50u8]);
                }
            }

            // imgbuf.save("test.png").unwrap();
            // display_image("", &imgbuf, 500, 500);
            Some(imgbuf)
        }
        Err(e) => None,
    }
}