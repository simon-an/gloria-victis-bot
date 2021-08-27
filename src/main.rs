use inputbot::handle_input_events;
#[allow(dead_code)]
mod screen;

mod game_interactions;
mod gui;
mod support;
mod tes;
mod shared;
pub mod statics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

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
