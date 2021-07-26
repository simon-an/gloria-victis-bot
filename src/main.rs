use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};
#[allow(dead_code)]
mod screen;

fn main() {

    let running: Arc<Mutex<bool>>= Arc::new(Mutex::new(true));

    // Rapidfire for videogames.
    let x = Arc::clone(&running);
    F5Key.bind(move || {
        *x.lock().unwrap() = true;
        while *x.lock().unwrap() {
            println!("Give me my water");
            RKey.press();
            sleep(Duration::from_millis(50));
            RKey.release();
            sleep(Duration::from_millis(2100));
        }
    });

    let y = Arc::clone(&running);
    F6Key.bind(move || {
        println!("i have enought water");
        *y.lock().unwrap() = false;
    });

    let r3 = Arc::clone(&running);
    F7Key.bind(move || {
        *r3.lock().unwrap() = true;
        while *r3.lock().unwrap() {
            println!("Give me my wood");
            MiddleButton.press();
            sleep(Duration::from_millis(450));
            MiddleButton.release();
            sleep(Duration::from_millis(700));
        }
    });

    let r4 = Arc::clone(&running);
    F8Key.bind(move || {
        println!("i have enought wood");
        *r4.lock().unwrap() = false;
    });

    let r5 = Arc::clone(&running);
    F9Key.bind(move || {
        *r5.lock().unwrap() = true;
        while *r5.lock().unwrap() {
            println!("Give me my iron");
            MiddleButton.press();
            sleep(Duration::from_millis(450));
            MiddleButton.release();
            sleep(Duration::from_millis(900));
        }
    });

    let r6 = Arc::clone(&running);
    F10Key.bind(move || {
        println!("i have enought iron");
        *r6.lock().unwrap() = false;
    });





    // Move mouse.
    // F7Key.bind(|| MouseCursor::move_rel(100, 0));

    // Call this to start listening for bound inputs.
    handle_input_events();
}