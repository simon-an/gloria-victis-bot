use inputbot::{KeybdKey::*, MouseButton, MouseCursor};
use std::{sync::Arc, thread::sleep, time::Duration};

use crate::{statics::*, Resolution};

pub fn run(window_pos: (i32, i32), resolution: Resolution) {
    let r = Arc::clone(&RUNNING);
    let c = Arc::clone(&COUNTER);
    let target = Arc::clone(&COUNTER_TARGET);

    F9Key.bind(move || {
        let (x, y) = MouseCursor::pos();

        *c.lock().unwrap() = 0;
        *r.lock().unwrap() = true;
        while *r.lock().unwrap() && *c.lock().unwrap() < *target.lock().unwrap() {
            // open menu on item
            // match resolution {
            //     Resolution::TWICE_QHD => MouseCursor::move_abs(600, 640),
            //     Resolution::SMALLEST =>  MouseCursor::move_abs(window_pos.0 + 310, window_pos.1 + 370),
            //     Resolution::FULL_HD  =>  MouseCursor::move_abs(window_pos.0 + 440, window_pos.1 + 500),
            //     _ => MouseCursor::move_abs(window_pos.0 +440, window_pos.1 + 500),
            // }
            sleep(Duration::from_millis(50));
            MouseButton::RightButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::RightButton.release();
            sleep(Duration::from_millis(50));

            // Recycle Menu Item
            match resolution {
                Resolution::TWICE_QHD => MouseCursor::move_rel(100, 140),
                Resolution::FULL_HD => MouseCursor::move_rel(100, 100),
                _ => MouseCursor::move_rel(100, 50),
            }

            sleep(Duration::from_millis(500));
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();
            sleep(Duration::from_millis(50));

            // if *c.lock().unwrap() == 0 {
                // Confirm Button
                match resolution {
                    Resolution::TWICE_QHD => MouseCursor::move_abs(2420, 730),
                    Resolution::SMALLEST => MouseCursor::move_abs(440, 420),
                    _ => MouseCursor::move_abs(window_pos.0 + 850, window_pos.1 + 580),
                }
            // } 

            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();
            sleep(Duration::from_millis(500));

            MouseCursor::move_abs(x, y);
            *c.lock().unwrap() += 1;
        }
    });
}
