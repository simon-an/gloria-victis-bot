mod craftloop;
mod salvage;
use inputbot::{KeybdKey::*, MouseButton::{self, *}, MouseCursor};

use std::{sync::Arc, thread::sleep, time::Duration};

use crate::{Resolution, statics::*};
use winapi::shared::windef::{
     RECT
};

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

pub(crate) fn bind_keys_for(mode: crate::shared::BotMode, window_pos: (i32, i32), resolution: Resolution) -> () {
    F9Key.unbind();
    F10Key.unbind();

    register_key_bindings();

    match mode {
        crate::shared::BotMode::PosCheck => {
            let r = Arc::clone(&RUNNING);
            F9Key.bind(move || {
                *r.lock().unwrap() = true;
                while *r.lock().unwrap() {

                    match resolution {
                        Resolution::TWICE_QHD => {

                        },
                        Resolution::FULL_HD => {

                        },
                        Resolution::SMALLEST => {
                            MouseCursor::move_abs(window_pos.0, window_pos.1);
                            sleep(Duration::from_millis(2000));
                            // MouseCursor::move_abs(window.right, window.bottom);
                            // sleep(Duration::from_millis(2000));
                            MouseCursor::move_abs(window_pos.0 + 572, window_pos.1 + 635);
                            sleep(Duration::from_millis(2000));
                            MouseCursor::move_abs(window_pos.0 + 1300, window_pos.1 + 680);
                            sleep(Duration::from_millis(2000));
                            MouseCursor::move_abs(window_pos.0 + 866, window_pos.1 + 580);
                            sleep(Duration::from_millis(2000));
                            MouseCursor::move_abs(window_pos.0 + 1050, window_pos.1 + 580);
                            sleep(Duration::from_millis(2000));
                        }
                        _ => {

                        }
                    }

                }
            });
          
        }
        crate::shared::BotMode::CraftLoop => {
            craftloop::run_craft_loop(window_pos, resolution);
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
            let target = Arc::clone(&COUNTER_TARGET);
            F9Key.bind(move || {
                *r.lock().unwrap() = true;
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
                while *r.lock().unwrap() && *c.lock().unwrap() < *target.lock().unwrap() {
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
            let target = Arc::clone(&COUNTER_TARGET);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                GKey.press();
                sleep(Duration::from_millis(50));
                GKey.release();
                sleep(Duration::from_millis(3000));
                while *r.lock().unwrap() && *c.lock().unwrap() < *target.lock().unwrap() {
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
            let target = Arc::clone(&COUNTER_TARGET);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                while *r.lock().unwrap() && *c.lock().unwrap() <  99999 {
                    GKey.press();
                    sleep(Duration::from_millis(50));
                    GKey.release();
                    while *r.lock().unwrap() && *c.lock().unwrap() <  *target.lock().unwrap() {
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
            salvage::run(window_pos, resolution);
         
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
