//! # ChunkWM plugin template in Rust
//! This template allows you to easily create plugins in Rust that use the ChunkWM events.
//!
//! To see the installation instructions, visit the
//! [README on GitHub](https://github.com/splintah/chunkwm-rs-template).

#[macro_use]
extern crate chunkwm;

use chunkwm::prelude::{CVar, Event, HandleEvent, Subscription, API};

// Create an event handler. Your handler should be `pub`.
pub struct Plugin {
    preselect_border_width: CVar<u32>,
    global_desktop_mode: CVar<String>,
}

// Create the bridge between the C/C++ plugin and the event handler.
create_c_bridge!(Plugin);

// Implement `HandleEvent` on the event handler.
impl HandleEvent for Plugin {
    fn new(api: &'static API) -> Plugin {
        println!("Rust template: Starting up...");
        // Add two CVars.
        let preselect_border_width = CVar::new("preselect_border_width", &api).unwrap();
        let global_desktop_mode = CVar::new("global_desktop_mode", &api).unwrap();
        Plugin {
            preselect_border_width,
            global_desktop_mode,
        }
    }

    // Specify name and version.
    name!("rust_template");
    version!("0.1.0");

    // Subscribe to events.
    subscribe!(
        Subscription::WindowFocused,
        Subscription::WindowMinimized,
        Subscription::ApplicationLaunched
    );

    // Handle events.
    fn handle(&mut self, event: Event) {
        match event {
            Event::WindowFocused(window) => {
                // NOTE(splintah): the printed text is printed to ChunkWM's stdout. When installing
                // via HomeBrew, you can use the options `--with-logging` or `--with-tmp-logging` to
                // redirect the stdout to a file. You could also write to your own file or process.
                println!(
                    "Rust template: \"{} - {}\" focused",
                    window.owner.name, window.name
                );
            }
            Event::WindowMinimized(window) => {
                println!(
                    "Rust template: \"{} - {}\" minimized",
                    window.owner.name, window.name
                );
            }
            Event::DaemonCommand(_) => {
                // Print CVars on daemon command.
                println!("{}", self.preselect_border_width.get_value().unwrap());
                println!("{}", self.global_desktop_mode.get_value().unwrap());
            }
            _ => (),
        }
    }

    // Shutdown the handler.
    fn shutdown(&self) {}
}
