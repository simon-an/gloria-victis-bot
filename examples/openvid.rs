use std::path::PathBuf;

use opencv::{
    highgui::{self, WINDOW_AUTOSIZE, WINDOW_GUI_EXPANDED, WINDOW_KEEPRATIO},
    prelude::*,
    video,
    videoio::{self, CAP_PROP_POS_FRAMES},
};
fn main() -> anyhow::Result<()> {
    let window = "video capture";
    highgui::named_window(
        window,
        WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED,
    )?;
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("GV.mp4");
    println!("path: {:?}", path);
    let cam =
        opencv::videoio::VideoCapture::from_file(path.to_str().unwrap(), opencv::videoio::CAP_ANY); // 0 is the default camera
    match cam {
        Ok(mut cam) => {
            let frames = cam.get(CAP_PROP_POS_FRAMES)?;
            println!("frames: {}", frames);
            let mut frame = Mat::default();
            cam.read(&mut frame)?;
            highgui::imshow(window, &mut frame)?;
        }
        Err(err) => println!("err {}", err),
    }
    Ok(())
}
