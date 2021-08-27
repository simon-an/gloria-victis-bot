use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

// Move inventory to top left and crafting window right to it. Select resipe and press F5 to start.
fn main() {
    let running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    let r1 = Arc::clone(&running);
    F5Key.bind(move || {
        *r1.lock().unwrap() = true;
        while *r1.lock().unwrap() {

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

    let r2 = Arc::clone(&running);
    F10Key.bind(move || {
        *r2.lock().unwrap() = false;
    });


    let left_jump = 72;
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);
    // sleep(Duration::from_secs(1));
    // MouseCursor::move_rel(left_jump, 0);

    // Call this to start listening for bound inputs.
    handle_input_events();
}
