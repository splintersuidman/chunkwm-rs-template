// # ChunkWM plugin template in Rust
// This template allows you to easily create plugins in Rust that use the ChunkWM events.
//
// To see the installation instructions, visit the
// [README on GitHub](https://github.com/splintah/chunkwm-rs-template).

#[macro_use] extern crate chunkwm;
// The bridge module defines the create_bridge!($struct) macro.
#[macro_use] mod bridge;

use chunkwm::prelude::{CVar, ChunkWMError, Event, HandleEvent, LogLevel, NumericBool,
                       Subscription, API};

// Create an event handler. Your handler should be `pub`.
pub struct Plugin {
    api: API,
    preselect_border_width: CVar<u32>,
    global_desktop_mode: CVar<String>,
    bsp_spawn_left: CVar<NumericBool>,
}

// Create the bridge between the C/C++ plugin and the event handler.
create_bridge!(Plugin);

// Implement `HandleEvent` on the event handler.
impl HandleEvent for Plugin {
    fn new(api: API) -> Plugin {
        println!("Rust template: Starting up...");
        Plugin {
            api,
            preselect_border_width: CVar::new("preselect_border_width", api).unwrap(),
            global_desktop_mode: CVar::new("global_desktop_mode", api).unwrap(),
            bsp_spawn_left: CVar::new("bsp_spawn_left", api).unwrap(),
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
    fn handle(&mut self, event: Event) -> Result<(), ChunkWMError> {
        match event {
            Event::WindowFocused(window) => {
                // NOTE(splintah): the printed text is printed to ChunkWM's stdout. You can use
                // `chunkc core::log_level <debug | warn | error>` to specify the desired log level,
                // and `chunkc core::log_file <stdout | stderr | /path/to/file>` for the desired log
                // file.
                self.api.log(
                    LogLevel::Debug,
                    format!(
                        "Rust template: \"{} - {}\" focused",
                        window.owner()?.name()?,
                        window.name()?
                    ),
                );
            }
            Event::WindowMinimized(window) => {
                self.api.log(
                    LogLevel::Debug,
                    format!(
                        "Rust template: \"{} - {}\" minimized",
                        window.owner()?.name()?,
                        window.name()?
                    ),
                );
            }
            Event::DaemonCommand(_) => {
                // Print CVars on daemon command (i.e. `chunkc template::command`).
                self.api.log(
                    LogLevel::Debug,
                    format!("Rust template: {}", self.preselect_border_width.value()?),
                );
                self.api.log(
                    LogLevel::Debug,
                    format!("Rust template: {}", self.global_desktop_mode.value()?),
                );
                self.api.log(
                    LogLevel::Debug,
                    format!("Rust template: {}", self.bsp_spawn_left.value()?.value),
                );

                // You can log using the chunkwm logging system. Use the LoggingLevel to specify the
                // output file; Debug: chunkwm.out.log, Warn and Error: chunkwm.err.log.
                self.api.log(LogLevel::Debug, "Rust template: DEBUG");
                self.api.log(LogLevel::Warn, "Rust template: WARN");
                self.api.log(LogLevel::Error, "Rust template: ERROR");
            }
            _ => (),
        };

        Ok(())
    }

    // Shutdown the handler.
    fn shutdown(&self) {
        self.api
            .log(LogLevel::Debug, "Rust template: shutting down.")
    }
}
