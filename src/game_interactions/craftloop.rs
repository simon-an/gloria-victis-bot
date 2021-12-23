use inputbot::{KeybdKey::*, MouseButton, MouseCursor};
use std::{sync::Arc, thread::sleep, time::Duration};
use winapi::shared::windef::{
    RECT
};
use crate::Resolution;

// Move inventory to top left and crafting window right to it. Select resipe and press F5 to start.
pub fn run_craft_loop(window_pos: (i32, i32), resolution: Resolution) {
    let r = Arc::clone(&crate::statics::RUNNING);

    F9Key.bind(move || {
        *r.lock().unwrap() = true;
        while *r.lock().unwrap() {
            // Craft Button
            
            match resolution {
                Resolution::TWICE_QHD => MouseCursor::move_abs(1750, 880),
                Resolution::SMALLEST =>  MouseCursor::move_abs(window_pos.0 + 310, window_pos.1 + 370),
                Resolution::FULL_HD  =>  MouseCursor::move_abs(window_pos.0 + 1300, window_pos.1 + 680),
                _ =>MouseCursor::move_abs(1300, 680),
            }
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();

            sleep(Duration::from_secs(3));

            // select invetory top left item
            match resolution {
                Resolution::TWICE_QHD => MouseCursor::move_abs(600, 640),
                Resolution::SMALLEST =>  MouseCursor::move_abs(window_pos.0 + 310, window_pos.1 + 370),
                Resolution::FULL_HD  =>  MouseCursor::move_abs(window_pos.0 + 440, window_pos.1 + 500),
                _ => MouseCursor::move_abs(window_pos.0 +440, window_pos.1 + 500),
            }
            sleep(Duration::from_millis(50));
            MouseButton::RightButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::RightButton.release();
            sleep(Duration::from_millis(50));

            // Recycle Menu Item
            MouseCursor::move_rel(100, 170);
            sleep(Duration::from_millis(500));
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();
            sleep(Duration::from_millis(50));

            // Confirm Button
            match resolution {
                Resolution::TWICE_QHD =>  MouseCursor::move_abs(2420, 730),
                Resolution::SMALLEST =>  MouseCursor::move_abs(440, 420),
                _ =>   MouseCursor::move_abs(window_pos.0 + 850, window_pos.1 + 580),
            }
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();
            sleep(Duration::from_millis(50));
        }
    });
}
