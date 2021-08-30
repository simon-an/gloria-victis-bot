use opencv::{
    core::{self, bitwise_not, Point2d, UMat, UMatUsageFlags, Vector},
    highgui, imgcodecs,
    imgproc::{
        self, AdaptiveThresholdTypes, ContourApproximationModes, RetrievalModes, ThresholdTypes,
        ADAPTIVE_THRESH_GAUSSIAN_C, ADAPTIVE_THRESH_MEAN_C, CHAIN_APPROX_SIMPLE, HOUGH_GRADIENT,
        LINE_AA, RETR_LIST, THRESH_BINARY, THRESH_BINARY_INV, THRESH_OTSU, THRESH_TOZERO,
        THRESH_TOZERO_INV, THRESH_TRIANGLE,
    },
    prelude::*,
    types::{self, VectorOfPoint, VectorOfPoint2d},
    Result,
};
use tesseract::{OcrEngineMode, Tesseract};

fn main() -> anyhow::Result<()> {
    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::env::set_var("TESSDATA_PREFIX", format!("{}\\{}", folder, "tessdata"));
    println!("{}", folder);

    // adaptiveThreshold, findContours,

    let img = imgcodecs::imread("images/32x10.jpg", imgcodecs::IMREAD_COLOR)?;
    // let img = imgcodecs::imread("images/Screenshot 2021-08-27 221642.jpg", imgcodecs::IMREAD_COLOR)?;

    // let rect = highgui::select_roi(&img, true, true)?;
    // let rect = core::Rect {  x: 584, y: 678, width: 178, height: 236 };
    let rect = core::Rect {
        x: 1905,
        y: 736,
        width: 478,
        height: 472,
    };
    println!("{:?}", rect);
    let img = Mat::roi(&img, rect)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let window1 = "adaptive_threshold";
    let window2 = "1THRESH_OTSU";
    let window3 = "2THRESH_TRIANGLE";
    highgui::named_window(window1, highgui::WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, highgui::WINDOW_AUTOSIZE)?;
    highgui::named_window(window3, highgui::WINDOW_AUTOSIZE)?;
    let mut text = Mat::default();
    opencv::imgproc::adaptive_threshold(
        &gray,
        &mut text,
        255f64,
        ADAPTIVE_THRESH_GAUSSIAN_C,
        THRESH_BINARY,
        3,
        15.0,
    )?;
    highgui::imshow(window1, &text)?;
    let mut text2 = Mat::default();
    opencv::imgproc::threshold(
        &gray,
        &mut text2,
        0f64,
        255f64,
        THRESH_BINARY_INV | THRESH_OTSU,
    )?;
    highgui::imshow(window2, &text2)?;
    let mut text3 = Mat::default();
    opencv::imgproc::threshold(
        &gray,
        &mut text3,
        0f64,
        255f64,
        THRESH_BINARY_INV | THRESH_TRIANGLE,
    )?;
    highgui::imshow(window3, &text3)?;
    highgui::wait_key(10000).unwrap();

    let temp = Tesseract::new_with_oem(None, Some("deu"), OcrEngineMode::Default)?;
    let frame = text3.data_typed::<u8>()?;
    let only_lstm_str = temp
        .set_frame(&frame, text.cols(), text.rows(), 1i32, text.cols())?
        .recognize()?
        .get_text()?;
    println!("{}", only_lstm_str);
    assert!(only_lstm_str.contains("Absenken"));

    Ok(())
}
