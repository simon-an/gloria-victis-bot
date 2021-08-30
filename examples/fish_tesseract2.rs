use opencv::{
    core::{self, bitwise_not, UMat, UMatUsageFlags},
    highgui, imgcodecs,
    imgproc::{self, ThresholdTypes, THRESH_BINARY, THRESH_BINARY_INV, THRESH_OTSU},
    prelude::*,
    types, Result,
};
use tesseract::{OcrEngineMode, Tesseract};

fn main() -> anyhow::Result<()> {
    // std::env::set_var("TESSDATA_PREFIX", format!("{}\\{}", folder, "tessdata"));
    let vec = vec![
        "images/Screenshot 2021-08-27 221424.jpg",
        "images/Screenshot 2021-08-27 221559.jpg",
        "images/Screenshot 2021-08-27 221617.jpg",
        "images/Screenshot 2021-08-27 221642.jpg",
    ];
    let results = vec![
        "Hoch ziehen",
        "Nach rechts ziehen",
        "Nach links ziehen",
        "Absenken",
    ];
    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", folder);

    let mut matches: Vec<Vec<bool>> = vec![];

    for t in 1..22 {
        let threshhold = (t * 10) as f64;
        let mut res: Vec<bool> = vec![];

        for i in 0..4 {
            let img = imgcodecs::imread(vec[i], imgcodecs::IMREAD_COLOR)?;
            let mut gray = Mat::default();
            imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

            let mut text = Mat::default();
            // opencv::imgproc::threshold(&gray, &mut text, 200f64, 255f64, THRESH_BINARY_INV | THRESH_OTSU)?;
            opencv::imgproc::threshold(
                &gray,
                &mut text,
                threshhold,
                255f64,
                THRESH_BINARY_INV | THRESH_OTSU,
            )?;

            let temp = Tesseract::new_with_oem(None, Some("deu_frak"), OcrEngineMode::Default)?;
            let frame = text.data_typed::<u8>()?;
            let only_lstm_str = temp
                .set_frame(&frame, text.cols(), text.rows(), 1i32, text.cols())?
                // let only_lstm_str = temp.set_image("x2.tiff")?
                .recognize()?
                .get_text()?;
            if i == 3 {
                println!("{}", only_lstm_str);
                let window = "frame";
                highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
                highgui::imshow(window, &text)?;
                highgui::wait_key(10000).unwrap();
            }
            // assert!(only_lstm_str.contains(results[i]));
            let r = only_lstm_str.contains(results[i]);
            println!("threshhold {:?}: {}", threshhold, r);
            res.push(r);
        }
        matches.push(res);
    }

    println!("Res: {:?}", matches);

    Ok(())
}
