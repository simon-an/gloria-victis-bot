#[macro_use]
extern crate winapi;

use image::DynamicImage;
use inputbot::handle_input_events;
use tokio::sync::mpsc::*;

#[allow(dead_code)]
mod screen;

mod app;
pub mod desktop;
mod game_interactions;
mod gui;
mod shared;
pub mod statics;
mod support;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Resolution {
    FULL_HD,
    TWICE_QHD,
    SMALLEST,
    ANY,
}
impl From<(i32, i32)> for Resolution {
    fn from((w, h): (i32, i32)) -> Self {
        let resolution = match (w, h) {
            (5120, 1440) => Resolution::TWICE_QHD,
            (1936, 1119) => Resolution::FULL_HD,
            (1040, 807) => Resolution::SMALLEST,
            _ => Resolution::ANY,
        };
        resolution
    }
}

#[cfg(feature = "fishing")]
mod tes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let (tx, rx) : (Sender<DynamicImage>, Receiver<DynamicImage>)= channel(2);

    #[cfg(feature = "fishing")]
    tokio::spawn(async move {
        tes::run(tx).await.unwrap();
    });

    tokio::spawn(async move {
        handle_input_events();
    });

    game_interactions::register_key_bindings();

    // Call this to start listening for bound inputs.

    // tokio::spawn(async move {
    //  gui::create();
    // });

    // tokio::spawn(async move {
    app::create(rx);
    // });

    Ok(())
}
