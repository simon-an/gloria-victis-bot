use opencv::{
    core::{self, bitwise_not},
    highgui::{self, WINDOW_AUTOSIZE, WINDOW_GUI_EXPANDED, WINDOW_KEEPRATIO},
    imgproc::{self, THRESH_BINARY_INV},
    prelude::*,
    videoio,
};
use tesseract::{OcrEngineMode, Tesseract};

use crate::game_interactions::{self, *};

pub async fn run() -> anyhow::Result<()> {
    let window = "video capture";
    highgui::named_window(
        window,
        WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED,
    )?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    cam.set(3, 1920f64)?;
    cam.set(4, 1080f64)?;
    let opened = cam.is_opened()?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let folder = format!("{}/{}", folder, "tessdata");
    println!("{}", folder);
    std::env::set_var("TESSDATA_PREFIX", folder);

    let mut previous_text = "".to_string();
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // highgui::imshow(window, &frame)?;

            // let window = "demo";
            // highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

            let mut gray = Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
            let mut blurred = Mat::default();
            // highgui::imshow(window, &gray)?;
            // highgui::wait_key(10000).unwrap();
            imgproc::gaussian_blur(
                &gray,
                &mut blurred,
                core::Size::new(7, 7),
                1.5,
                0.,
                core::BORDER_DEFAULT,
            )?;

            let mut edges = Mat::default();
            imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;

            let empty_mat = Mat::default();
            let mut text = Mat::default();
            let mut text2 = Mat::default();
            bitwise_not(&gray, &mut text, &empty_mat)?;

            opencv::imgproc::threshold(&text, &mut text2, 50f64, 255f64, THRESH_BINARY_INV)?;

            let frame = text2.data_typed::<u8>()?;
            // println!("{:?}", frame);

            // println!("{:?} {:?} {:?} {:?} {:?}", text2.cols(), text2.rows(), text2.dims(), text2.typ(), text2.channels().unwrap());
            // highgui::imshow(window, &text2)?;
            // highgui::wait_key(1000).unwrap();
            let temp = Tesseract::new_with_oem(None, Some("deu_frak"), OcrEngineMode::Default)?;
            let only_lstm_str = temp
                .set_frame(&frame, text2.cols(), text2.rows(), 1i32, text2.cols())?
                .recognize()?
                .get_text()?
                .trim()
                .to_string();
            //  println!("{}", only_lstm_stt);
            if only_lstm_str != previous_text {
                previous_text = only_lstm_str.clone();
                let dir: Option<Direction> = match only_lstm_str.as_str() {
                    "Nach rechts Zizhen" => Some(Direction::Right),
                    "Absenken" => Some(Direction::Down),
                    "Nach links Zizhen" => Some(Direction::Left),
                    "Hoch Zizhen" => Some(Direction::Up),
                    _ => None,
                };

                if let Some(dir) = dir {
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

    Ok(())
}
