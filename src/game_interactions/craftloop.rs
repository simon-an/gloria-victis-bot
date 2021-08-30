use inputbot::{KeybdKey::*, MouseButton, MouseCursor};
use std::{sync::Arc, thread::sleep, time::Duration};

// Move inventory to top left and crafting window right to it. Select resipe and press F5 to start.
pub fn run_craft_loop() {
    let r = Arc::clone(&crate::statics::RUNNING);

    F9Key.bind(move || {
        *r.lock().unwrap() = true;
        while *r.lock().unwrap() {
            // Craft Button
            MouseCursor::move_abs(1750, 880);
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();

            sleep(Duration::from_secs(3));

            // select invetory top left item
            MouseCursor::move_abs(600, 640);
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
            MouseCursor::move_abs(2420, 730);
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.press();
            sleep(Duration::from_millis(50));
            MouseButton::LeftButton.release();
            sleep(Duration::from_millis(50));
        }
    });
}
