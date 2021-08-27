mod craftloop;
use inputbot::{KeybdKey::*, MouseButton::*, *};

use std::{sync::Arc, thread::sleep, time::Duration};

use crate::statics::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn fishing(what_to_do: Direction) {
    match what_to_do {
        Direction::Up => {
            MouseCursor::move_rel(0, -10);
            println!("move up");
        }
        Direction::Down => {
            println!("move down");
            MouseCursor::move_rel(0, 10);
        }
        Direction::Left => {
            println!("move left");
            MouseCursor::move_rel(-10, 0);
        }
        Direction::Right => {
            println!("move right");
            MouseCursor::move_rel(10, 0);
        }
    };
    MouseButton::LeftButton.press();
    sleep(Duration::from_millis(50));
    MouseButton::LeftButton.release();
}

pub fn register_key_bindings() {
    let r6 = Arc::clone(&RUNNING);
    let c3 = Arc::clone(&COUNTER);
    F10Key.bind(move || {
        println!("i have enought");
        *r6.lock().unwrap() = false;
        *c3.lock().unwrap() = 0;
    });
}

pub(crate) fn bind_keys_for(mode: crate::shared::BotMode) -> () {
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
                FKey.press();
                sleep(Duration::from_millis(50));
                FKey.release();
                while *r.lock().unwrap() && *c.lock().unwrap() < 50 {
                    println!("Give me my wood {}", *c.lock().unwrap());
                    MiddleButton.press();
                    sleep(Duration::from_millis(450));
                    MiddleButton.release();
                    sleep(Duration::from_millis(700));
                    *c.lock().unwrap() += 1;
                }
                FKey.press();
                sleep(Duration::from_millis(50));
                FKey.release();
            });
        }
        crate::shared::BotMode::Iron => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                FKey.press();
                sleep(Duration::from_millis(50));
                FKey.release();
                while *r.lock().unwrap() && *c.lock().unwrap() < 18 {
                    println!("Give me my iron {}", *c.lock().unwrap());
                    MiddleButton.press();
                    sleep(Duration::from_millis(420));
                    MiddleButton.release();
                    sleep(Duration::from_millis(900));
                    *c.lock().unwrap() += 1;
                }
                FKey.press();
                sleep(Duration::from_millis(50));
                FKey.release();
            });
        }
        crate::shared::BotMode::ManyIron => {
            let r = Arc::clone(&RUNNING);
            let c = Arc::clone(&COUNTER);
            F9Key.bind(move || {
                *c.lock().unwrap() = 0;
                *r.lock().unwrap() = true;
                while *r.lock().unwrap() && *c.lock().unwrap() < 18 {
                    FKey.press();
                    sleep(Duration::from_millis(50));
                    FKey.release();
                    while *c.lock().unwrap() < 18 {
                        println!("Give me my iron {}", *c.lock().unwrap());
                        MiddleButton.press();
                        sleep(Duration::from_millis(420));
                        MiddleButton.release();
                        sleep(Duration::from_millis(900));
                        *c.lock().unwrap() += 1;
                    }
                    *c.lock().unwrap() = 0;
                    FKey.press();
                    sleep(Duration::from_millis(50));
                    FKey.release();
                    sleep(Duration::from_millis(8000));
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
