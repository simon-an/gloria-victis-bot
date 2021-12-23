use std::{path::PathBuf, time::Duration};
use std::ops::Deref;
use opencv::core::{BORDER_CONSTANT, BorderTypes, bitwise_and};
use opencv::{
    core::{self, Scalar, Size, Vec3b},
    features2d::{draw_keypoints, ORB},
    highgui, imgcodecs,
    imgproc::{self, *},
    prelude::*,
    types::VectorOfKeyPoint,
    Result,
};

fn main() -> anyhow::Result<()> {
    let window = "demo";
    let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/large_up.jpg");
    let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    highgui::imshow(window, &img)?;
    highgui::wait_key(10000).unwrap();

    let matDataVec = match Mat::data_typed::<Vec3b>(&img) {
        Ok(matData) => matData,
        Err(matDataErr) => {
            anyhow::bail!("bla")
        }
    };

    let pixVal: &[u8; 3] = Vec3b::deref(&matDataVec[0]);
    println!("{:?}", pixVal);
    // redPixels[i] = pixVal[2];
    // greenPixels[i] = pixVal[1];
    // bluePixels[i] = pixVal[0];
    let lower_bound = Scalar::from((100.0, 0.0, 0.0));
    let upper_bound = Scalar::from((255.0, 150.0, 73.0));
    let mut mask = Mat::default();
    
    opencv::core::in_range(&img, &lower_bound, &upper_bound, &mut mask).unwrap();
    highgui::imshow(window, &mask)?;
    highgui::wait_key(10000).unwrap();
    
    let mut mask2 = Mat::default();
    let kernel = Mat::default();
    let point = core::Point { x: -1, y:-1  };
    erode(&mask, &mut mask2, &kernel, point ,3, BORDER_CONSTANT, core::Scalar::all(-1f64))?;
    highgui::imshow(window, &mask2)?;
    highgui::wait_key(10000).unwrap();
    
    let mut mask3 = Mat::default();
    let point = core::Point { x: -1, y:-1  };
    dilate(&mask2, &mut mask3,  &kernel, point ,4, BORDER_CONSTANT, core::Scalar::all(-1f64))?;
    highgui::imshow(window, &mask3)?;
    highgui::wait_key(10000).unwrap();
    
    
    // let mut edges = Mat::default();
    // imgproc::canny(&mask, &mut edges, 0., 50., 3, false)?;
    // highgui::imshow(window, &edges)?;
    // highgui::wait_key(10000).unwrap();

    let mut color_mask = Mat::default();
    imgproc::cvt_color(&mask, &mut color_mask, imgproc::COLOR_GRAY2BGR, 0)?;

    let empty_mat = Mat::default();
    let mut res = Mat::default();
    bitwise_and(&img, &color_mask, &mut res, &empty_mat)?;
    highgui::imshow(window, &res)?;
    highgui::wait_key(10000).unwrap();

//     img,cnts,hie = cv2.findContours(mask,cv2.RETR_TREE,cv2.CHAIN_APPROX_NONE)
// cv2.drawContours(frame,cnts,-1,(0,255,0),3)


 // COPY ROI
//     let mut image_bigger = Mat::new_rows_cols_with_default(5, 5, u8::typ(), Scalar::all(10.))?;
// let mut image_bigger_roi = Mat::roi(&image_bigger, Rect::new(1, 1, 3, 3))?;
// let image_small = Mat::new_rows_cols_with_default(3, 3, u8::typ(), Scalar::all(3.))?;
// image_small.copy_to(&mut image_bigger_roi)?;

    Ok(())
}
