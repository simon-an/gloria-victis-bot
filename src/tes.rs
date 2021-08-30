use opencv::{
    core::{self, bitwise_not, Vec3b},
    highgui::{self, WINDOW_AUTOSIZE, WINDOW_GUI_EXPANDED, WINDOW_KEEPRATIO},
    imgproc::{self, THRESH_BINARY_INV, THRESH_OTSU},
    prelude::*,
    videoio,
};
use tesseract::{OcrEngineMode, Tesseract};
use winapi::shared::minwindef::UCHAR;

use crate::game_interactions::{self, *};

pub async fn run() -> anyhow::Result<()> {
    // let window = "video capture";
    // highgui::named_window(
    //     window,
    //     WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED,
    // )?;
    let mut cam = videoio::VideoCapture::new(1, videoio::CAP_ANY)?; // 0 is the default camera
    cam.set(3, 2560.0)?;
    // cam.set(3, 5120f64)?;
    cam.set(4, 1440f64)?;
    // cam.set(3, 1920f64)?;
    // cam.set(4, 1080f64)?;
    let opened = cam.is_opened()?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let folder = format!("{}/{}", folder, "tessdata");
    println!("{}", folder);
    std::env::set_var("TESSDATA_PREFIX", folder);
    let mut roi_selected = false;
    // let mut rect = None;
    let mut previous_text = "".to_string();
    // Full HD??
    // let rect = core::Rect {  x: 584, y: 678, width: 178, height: 236 };
    // 2560x1440
    let rect = Some(core::Rect {
        x: 700,
        y: 830,
        width: 320,
        height: 326,
    });
    // 5120 x 1440
    // let rect = core::Rect {   x: 1905, y: 736, width: 478, height: 472 };
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // let pixel = frame.at_2d::<Vec3b>(877,961)?;
            // println!("pixel: {:?}", pixel);

            // let point = core::Point { x: 877, y:961  };
            // imgproc::circle(
            //     	&mut frame,
            //     	point,
            //     	10,
            //     	core::Scalar::new(0f64, -1f64, -1f64, -1f64),
            //     	3,
            //     	0, 0)?;

            // imgproc::rectangle(
            // 	&mut frame,
            // 	rect.unwrap(),
            // 	core::Scalar::new(0f64, -1f64, -1f64, -1f64),
            // 	5,
            // 	8,
            // 	0)?;
            // highgui::imshow(window, &frame)?;
            // highgui::wait_key(10000)?;
            // highgui::destroy_window(window);

            // if !roi_selected {
            //     // highgui::imshow(window, &frame)?;
            // rect = Some(highgui::select_roi(&frame, true, true)?);
            // println!("{:?}", rect);
            //     roi_selected = true;
            // }

            if let Some(rect) = rect {
                // println!("{:?}", rect);
                let img = Mat::roi(&frame, rect)?;

                let mat = pre2(img)?;

                let frame = mat.data_typed::<u8>()?;

                // println!("{:?}", frame);

                // println!("{:?} {:?} {:?} {:?} {:?}", text2.cols(), text2.rows(), text2.dims(), text2.typ(), text2.channels().unwrap());
                // highgui::imshow(window, &mat)?;
                // highgui::wait_key(10).unwrap();
                let temp = Tesseract::new_with_oem(None, Some("deu"), OcrEngineMode::Default)?;
                let only_lstm_str = temp
                    .set_frame(&frame, mat.cols(), mat.rows(), 1i32, mat.cols())?
                    .recognize()?
                    .get_text()?
                    .trim()
                    .to_string();
                // log::info!("{} - {}", only_lstm_str, previous_text);
                if only_lstm_str != previous_text {
                    previous_text = only_lstm_str.clone();
                    let dir = if only_lstm_str.contains("Nach rechts ziehen") {
                        Some(Direction::Right)
                    } else if only_lstm_str.contains("Nach links ziehen") {
                        Some(Direction::Left)
                    } else if only_lstm_str.contains("Hoch ziehen") {
                        Some(Direction::Up)
                    } else if only_lstm_str.contains("Absenken") {
                        Some(Direction::Down)
                    } else if only_lstm_str.contains("Fischen erfolgreich!") {
                        log::info!("YESSS!");
                        None
                    } else if only_lstm_str.contains("Da ist etwas!") {
                        log::info!("Incoming!");
                        None
                    } else if only_lstm_str.contains("Fischgrund") {
                        log::info!("Waiting!");
                        None
                    } else {
                        None
                    };
                    // let dir: Option<Direction> = match only_lstm_str.as_str() {
                    //     "Nach rechts Ziehen" => Some(Direction::Right),
                    //     "Absenken" => Some(Direction::Down),
                    //     "Nach links Ziehen" => Some(Direction::Left),
                    //     "Hoch Ziehen" => Some(Direction::Up),
                    //     // Da Ist etwas!
                    //     _ => None,
                    // };

                    if let Some(dir) = dir {
                        log::trace!("Do something!");
                        game_interactions::fishing(dir);
                    } else {
                        log::trace!("Do nothing!");
                    }
                }
            }
            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
    }

    Ok(())
}

fn pre2(frame: Mat) -> anyhow::Result<Mat> {
    // let window = "frame";
    // highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut blurred = Mat::default();
    // highgui::imshow(window, &gray)?;
    // highgui::wait_key(10000).unwrap();
    // imgproc::gaussian_blur(
    //     &gray,
    //     &mut blurred,
    //     core::Size::new(7, 7),
    //     1.5,
    //     0.,
    //     core::BORDER_DEFAULT,
    // )?;

    let mut edges = Mat::default();
    imgproc::canny(&gray, &mut edges, 0., 50., 3, false)?;

    let empty_mat = Mat::default();
    let mut text = Mat::default();
    bitwise_not(&gray, &mut text, &empty_mat)?;
    // highgui::imshow(window, &text)?;

    let mut text2 = Mat::default();
    opencv::imgproc::threshold(&text, &mut text2, 85f64, 255f64, THRESH_BINARY_INV)?;
    let mut text3 = Mat::default();
    bitwise_not(&text2, &mut text3, &empty_mat)?;
    // highgui::imshow(window, &text3)?;
    Ok(text3)
}

fn pre1(img: Mat) -> anyhow::Result<Mat> {
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut text = Mat::default();
    // opencv::imgproc::threshold(&gray, &mut text, 200f64, 255f64, THRESH_BINARY_INV | THRESH_OTSU)?;
    opencv::imgproc::threshold(
        &gray,
        &mut text,
        200.0,
        255f64,
        THRESH_BINARY_INV | THRESH_OTSU,
    )?;
    Ok(text)
}
