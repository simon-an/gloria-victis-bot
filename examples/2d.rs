use std::{path::PathBuf, time::Duration};

use opencv::{
    core::Size,
    features2d::{draw_keypoints, ORB},
    highgui, imgcodecs,
    prelude::*,
    types::VectorOfKeyPoint,
    Result,
};

fn main() -> Result<()> {
    let window = "demo";
    let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/hoch.png");
    let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_ANYCOLOR)?;
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    highgui::imshow(window, &img)?;
    highgui::wait_key(10000).unwrap();
    let mut orb = <dyn ORB>::default()?;
    let mut kp = VectorOfKeyPoint::new();
    let mut des = Mat::default();
    orb.detect_and_compute(&img, &Mat::default(), &mut kp, &mut des, false)?;
    let size = if cfg!(ocvrs_opencv_branch_32) {
        35
    } else {
        35
    };
    assert_eq!(size, kp.len());
    assert_eq!(Size::new(32, size as i32), des.size()?);

    let mut res = Mat::default();
    draw_keypoints(
        &img,
        &kp,
        &mut res,
        opencv::core::Scalar::all(-1f64),
        opencv::features2d::DrawMatchesFlags::DEFAULT,
    )?;
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    highgui::imshow(window, &res)?;
    highgui::wait_key(10000).unwrap();
    Ok(())
}
