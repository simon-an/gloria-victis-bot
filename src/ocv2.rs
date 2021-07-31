use std::path::PathBuf;

use opencv::{Result, highgui::{self, WINDOW_AUTOSIZE, WINDOW_GUI_EXPANDED, WINDOW_KEEPRATIO}, imgcodecs, imgproc::{TM_CCOEFF, TM_SQDIFF_NORMED}, prelude::*, video, videoio};


pub async fn run ()-> anyhow::Result<()>{

    let window = "video capture";
	highgui::named_window(window, WINDOW_AUTOSIZE | WINDOW_KEEPRATIO | WINDOW_GUI_EXPANDED)?;
	// let mut cam = videoio::VideoCapture::new(0, videoio::CAP_FFMPEG)?; // 0 is the default camera
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("GV.mp4");
    println!("path: {:?}", path);
    // let file = tokio::fs::File::open(&path).await?;
	let mut cam = videoio::VideoCapture::from_file("GV.mp4", videoio::CAP_ANY)?; // 0 is the default camera
	// let r = cam.open_file(path.to_str().unwrap(), videoio::CAP_ANY)?;
    // println!("opend: {:?}", r);

    let opened = cam.is_opened()?;
	if !opened {
		panic!("Unable to open default camera!");
	}
    // let method = TM_SQDIFF_NORMED;
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/right.png");
    let templ = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    // let mut background = video::create_background_subtractor_knn(500, 400.0, true)?;
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
            // let mut result = Mat::default();
            // opencv::features2d::draw_matches(img1, keypoints1, img2, keypoints2, matches1to2, out_img, match_color, single_point_color, matches_mask, flags)
            // matchTemplate(&mat, &frame, method);
            // opencv::imgproc::match_template(&frame, &templ, &mut result, TM_CCOEFF, &Mat::default())?;

            highgui::imshow(window, &mut frame)?;
            // let mut fgmask = Mat::default();
			// background.apply(&frame, &mut fgmask, 0.1)?;
            // highgui::imshow(window, &mut fgmask)?;
		}
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
			break;
		}
	}
    Ok(())
}