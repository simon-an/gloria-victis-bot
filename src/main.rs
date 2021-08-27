use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};
#[allow(dead_code)]
mod screen;

// #[cfg(feature = "opencv1")]
mod ocv;
mod ocv2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // tokio::spawn(async move {
    //     ocv::run().await.unwrap();
    // });

    tokio::spawn(async move {
        ocv2::run().await.unwrap();
    });
    let running: Arc<Mutex<bool>>= Arc::new(Mutex::new(true));
    let counter: Arc<Mutex<u32>>= Arc::new(Mutex::new(0));

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

    let r3 = Arc::clone(&running);
    let c1 = Arc::clone(&counter);
    F6Key.bind(move || {
        *r3.lock().unwrap() = true;
        let mut counter =  *c1.lock().unwrap();
        counter = 0;
        Numrow8Key.press();
        sleep(Duration::from_millis(50));
        Numrow8Key.release();
        Numrow8Key.press();
        sleep(Duration::from_millis(50));
        Numrow8Key.release();
        while *r3.lock().unwrap() && counter < 50 {
            println!("Give me my wood {}", counter);
            MiddleButton.press();
            sleep(Duration::from_millis(50));
            MiddleButton.release();
            sleep(Duration::from_millis(700));
            counter += 1;
        }
        FKey.press();
        sleep(Duration::from_millis(50));
        FKey.release();
    });

    let r5 = Arc::clone(&running);
    let c2 = Arc::clone(&counter);
    F7Key.bind(move || {
        *c2.lock().unwrap() = 0;
        *r5.lock().unwrap() = true;
        Numrow9Key.press();
        sleep(Duration::from_millis(50));
        Numrow9Key.release();
        Numrow9Key.press();
        sleep(Duration::from_millis(50));
        Numrow9Key.release();
        while *r5.lock().unwrap()  && *c2.lock().unwrap() < 28 {
            println!("Give me my iron {}", *c2.lock().unwrap());
            MiddleButton.press();
            sleep(Duration::from_millis(50));
            MiddleButton.release();
            sleep(Duration::from_millis(2000));
            *c2.lock().unwrap() += 1;
        }
        FKey.press();
        sleep(Duration::from_millis(50));
        FKey.release();
    });

    let r7 = Arc::clone(&running);
    let c4 = Arc::clone(&counter);
    F8Key.bind(move || {
        *c4.lock().unwrap() = 0;
        *r7.lock().unwrap() = true;
        Numrow9Key.press();
        sleep(Duration::from_millis(50));
        Numrow9Key.release();
        Numrow9Key.press();
        sleep(Duration::from_millis(50));
        Numrow9Key.release();
        while *r7.lock().unwrap()  && *c4.lock().unwrap() < 18 {
            FKey.press();
            sleep(Duration::from_millis(50));
            FKey.release();
            while *r7.lock().unwrap() && *c4.lock().unwrap() < 30 {
                println!("Give me my iron {}", *c4.lock().unwrap());
                MiddleButton.press();
                sleep(Duration::from_millis(50));
                MiddleButton.release();
                sleep(Duration::from_millis(1600));
                *c4.lock().unwrap() += 1;
            }
            *c4.lock().unwrap() = 0;
            FKey.press();
            sleep(Duration::from_millis(50));
            FKey.release();
            sleep(Duration::from_millis(8000));
         }
    });

    let r6 = Arc::clone(&running);
    let c3 = Arc::clone(&counter);
    F10Key.bind(move || {
        println!("i have enought iron");
        *r6.lock().unwrap() = false;
        *c3.lock().unwrap() = 0;
    });





    // Move mouse.
    // F7Key.bind(|| MouseCursor::move_rel(100, 0));

    // Call this to start listening for bound inputs.
    handle_input_events();
    Ok(())
}