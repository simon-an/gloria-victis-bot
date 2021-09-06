mod craftloop;
use inputbot::{KeybdKey::*, MouseButton::{self, *}, MouseCursor};

use std::{sync::Arc, thread::sleep, time::Duration};

use crate::statics::*;

#[cfg(feature = "fishing")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(feature = "fishing")]
pub fn fishing(what_to_do: Direction) {
    match what_to_do {
        Direction::Up => {
            println!("move up");
            MouseButton::MiddleButton.press();
            sleep(Duration::from_millis(25));
            MouseButton::MiddleButton.release();
        }
        Direction::Down => {
            println!("move down");
            FKey.press();
            sleep(Duration::from_millis(25));
            FKey.release();
        }
        Direction::Left => {
            println!("move left");
            QKey.press();
            sleep(Duration::from_millis(25));
            QKey.release();

        }
        Direction::Right => {
            println!("move right");
            EKey.press();
            sleep(Duration::from_millis(25));
            EKey.release();
        }
    };
    // sleep(Duration::from_millis(1000));
}

pub fn register_key_bindings() {
    let r6 = Arc::clone(&RUNNING);
    let c3 = Arc::clone(&COUNTER);
    F10Key.bind(move || {
        println!("i have enough");
        *r6.lock().unwrap() = false;
        *c3.lock().unwrap() = 0;
    });
}

pub(crate) fn bind_keys_for(mode: crate::shared::BotMode, mouse: (i32, i32)) -> () {
    F9Key.unbind();

    match mode {
        crate::shared::BotMode::CraftLoop => {
            craftloop::run_craft_loop();
        }
        crate::shared::BotMode::Disabled => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            *r.lock().unwrap() = false;
            *c.lock().unwrap() = 0;
        }
        crate::shared::BotMode::Water => {
            let r = Arc::clone(&RUNNING);
            F9Key.bind(move || {
                *r.lock().unwrap() = true;
                while *r.lock().unwrap() {
                    println!("Give me my water");
                    RKey.press();
                    sleep(Duration::from_millis(50));
                    RKey.release();
                    sleep(Duration::from_millis(2100));
                }
            });
        }
        crate::shared::BotMode::Wood => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *r.lock().unwrap() = true;
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
                while *r.lock().unwrap() && *c.lock().unwrap() < 50 {
                    println!("Give me my wood {}", *c.lock().unwrap());
                    MiddleButton.press();
                    sleep(Duration::from_millis(450));
                    MiddleButton.release();
                    sleep(Duration::from_millis(700));
                    *c.lock().unwrap() += 1;
                }
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
            });
        }
        crate::shared::BotMode::Iron => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
                sleep(Duration::from_millis(3000));
                while *r.lock().unwrap() && *c.lock().unwrap() < 25 {
                    println!("Give me my iron {}", *c.lock().unwrap());
                    MiddleButton.press();
                    sleep(Duration::from_millis(20));
                    MiddleButton.release();
                    sleep(Duration::from_millis(1600));
                    *c.lock().unwrap() += 1;
                }
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
            });
        }
        crate::shared::BotMode::ManyIron => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                while *r.lock().unwrap() && *c.lock().unwrap() < 25 {
                    GKey.press();
                    sleep(Duration::from_millis(50));
                    GKey.release();
                    while *r.lock().unwrap() && *c.lock().unwrap() < 25 {
                        println!("Give me my iron {}", *c.lock().unwrap());
                        MiddleButton.press();
                        sleep(Duration::from_millis(20));
                        MiddleButton.release();
                        sleep(Duration::from_millis(1600));
                        *c.lock().unwrap() += 1;
                    }
                    *c.lock().unwrap() = 0;
                    GKey.press();
                    sleep(Duration::from_millis(50));
                    GKey.release();
                    sleep(Duration::from_millis(8000));
                }
            });
        }
        crate::shared::BotMode::Horse => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;

                while *r.lock().unwrap()
                {
                   LShiftKey.press();
                   WKey.press();
                   sleep(Duration::from_secs(30));
                   LShiftKey.release();
                   WKey.release();
                   DKey.press();
                   sleep(Duration::from_millis(1900));
                   DKey.release();
                }
                
            });
        }

        crate::shared::BotMode::SalvageItems  => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                let (x, y) = mouse;

                while *r.lock().unwrap() && *c.lock().unwrap() < 100
                {
                    // open menu on item
                    MouseCursor::move_abs(x, y);
                    sleep(Duration::from_millis(50));
                    RightButton.press();
                    sleep(Duration::from_millis(50));
                    RightButton.release();
                    sleep(Duration::from_millis(50));

                    // Recycle Menu Item
                    MouseCursor::move_rel(100, 170);
                    sleep(Duration::from_millis(500));
                    LeftButton.press();
                    sleep(Duration::from_millis(50));
                    LeftButton.release();
                    sleep(Duration::from_millis(50));

                    // Confirm Button
                    MouseCursor::move_abs(2420, 730);
                    sleep(Duration::from_millis(50));
                    LeftButton.press();
                    sleep(Duration::from_millis(50));
                    LeftButton.release();
                    sleep(Duration::from_millis(50));

                    *c.lock().unwrap() += 1;
                  
                }
                
            });
        }
        // ManyIron => {
        //     let r = Arc::clone(&RUNNING);
        //     let c = Arc::clone(&COUNTER);
        //     F9Key.bind(move || {
        // *c.lock().unwrap() = 0;
        // *r.lock().unwrap() = true;
        //     });
        // },
    }
}
