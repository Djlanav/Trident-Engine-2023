use std::process;
use trident_core::window_handling;

fn main() {
    let core = match window_handling::init_display() {
        Ok(core) => core,
        Err(error) => {
            eprintln!("Error occurred in making event loop: {error}");
            process::exit(1);
        }
    };

    window_handling::run_event_loop(core);
}
