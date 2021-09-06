use inputbot::handle_input_events;
#[allow(dead_code)]
mod screen;

mod game_interactions;
mod gui;
mod shared;
pub mod statics;
mod support;

#[cfg(feature = "fishing")]
mod tes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    #[cfg(feature = "fishing")]
    tokio::spawn(async move {
        tes::run().await.unwrap();
    });

    tokio::spawn(async move {
        handle_input_events();
    });

    game_interactions::register_key_bindings();

    // Call this to start listening for bound inputs.

    gui::create();
    Ok(())
}
