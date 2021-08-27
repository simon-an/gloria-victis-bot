use opencv::{
    core::{self, bitwise_not, UMat, UMatUsageFlags},
    highgui, imgcodecs,
    imgproc::{self, ThresholdTypes, THRESH_BINARY, THRESH_BINARY_INV, THRESH_OTSU},
    prelude::*,
    types, Result,
};
use tesseract::{OcrEngineMode, Tesseract};

fn main() -> anyhow::Result<()> {
    let window = "demo";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let img = imgcodecs::imread("images/rechts.png", imgcodecs::IMREAD_COLOR)?;
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut blurred = Mat::default();
    highgui::imshow(window, &gray)?;
    highgui::wait_key(10000).unwrap();
    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        core::Size::new(7, 7),
        1.5,
        0.,
        core::BORDER_DEFAULT,
    )?;
    println!("blured {:?}", gray.typ());
    highgui::imshow(window, &blurred)?;
    highgui::wait_key(10000).unwrap();

    let mut edges = Mat::default();
    imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
    highgui::imshow(window, &edges)?;
    highgui::wait_key(10000).unwrap();

    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", folder);
    let empty_mat = Mat::default();
    let mut text = Mat::default();
    let mut text2 = Mat::default();
    bitwise_not(&gray, &mut text, &empty_mat)?;

    opencv::imgproc::threshold(&text, &mut text2, 50f64, 255f64, THRESH_BINARY_INV)?;

    highgui::imshow(window, &text2)?;
    highgui::wait_key(10000).unwrap();
    // imgcodecs::imwrite("x2.tiff", &text2, &opencv::types::VectorOfi32::new())? ;

    std::env::set_var("TESSDATA_PREFIX", format!("{}\\{}", folder, "tessdata"));
    // let temp =  Tesseract::new_with_oem(None, Some("deu_frak"), OcrEngineMode::Default)?;
    // println!("{:?}", frame);
    // let only_lstm_str = temp.set_image("x.tiff")?
    // .recognize()?
    // .get_text()?;
    // println!("{}", only_lstm_str);
    let temp = Tesseract::new_with_oem(None, Some("deu_frak"), OcrEngineMode::Default)?;
    let frame = text2.data_typed::<u8>()?;
    let only_lstm_str = temp
        .set_frame(&frame, text2.cols(), text2.rows(), 1i32, text2.cols())?
        // let only_lstm_str = temp.set_image("x2.tiff")?
        .recognize()?
        .get_text()?;
    println!("{}", only_lstm_str);

    Ok(())
}
