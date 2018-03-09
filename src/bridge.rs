#[macro_export]
macro_rules! create_bridge {
    ($struct_ident:ident) => {pub mod __export {
        extern crate chunkwm;
        use super::$struct_ident;
        use chunkwm::prelude::*;
        use chunkwm::raw::ChunkWMPlugin;
        use std::os::raw::{c_char, c_uint, c_void};
        use std::ffi;

        static mut PLUGIN: Option<$struct_ident> = None;

        #[no_mangle]
        pub extern "C" fn chunkwm_rust_get_name() -> *const c_char {
            ffi::CString::new($struct_ident::name()).unwrap().into_raw()
        }

        #[no_mangle]
        pub unsafe extern "C" fn chunkwm_rust_get_version() -> *const c_char {
            ffi::CString::new($struct_ident::version())
                .unwrap()
                .into_raw()
        }

        pub extern "C" fn chunkwm_plugin_main(node: *const c_char, data: *mut c_void) -> bool {
            use chunkwm::event::DisplayID;
            let event = unsafe {
                ffi::CStr::from_ptr(node)
                    .to_string_lossy()
                    .into_owned()
            };

            let event = match event.as_str() {
                "chunkwm_export_application_launched" => {
                    Event::ApplicationLaunched(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_application_terminated" => {
                    Event::ApplicationTerminated(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_application_activated" => {
                    Event::ApplicationActivated(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_application_deactivated" => {
                    Event::ApplicationDeactivated(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_application_hidden" => {
                    Event::ApplicationHidden(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_application_unhidden" => {
                    Event::ApplicationUnhidden(Box::new((data as ApplicationRef).into()))
                }
                "chunkwm_export_window_created" => {
                    Event::WindowCreated(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_destroyed" => {
                    Event::WindowDestroyed(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_focused" => {
                    Event::WindowFocused(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_moved" => Event::WindowMoved(Box::new((data as WindowRef).into())),
                "chunkwm_export_window_resized" => {
                    Event::WindowResized(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_minimized" => {
                    Event::WindowMinimized(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_deminimized" => {
                    Event::WindowDeminimized(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_window_title_changed" => {
                    Event::WindowTitleChanged(Box::new((data as WindowRef).into()))
                }
                "chunkwm_export_display_added" => unsafe { Event::DisplayAdded(*(data as *mut DisplayID)) },
                "chunkwm_export_display_removed" => unsafe {
                    Event::DisplayRemoved(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_moved" => unsafe { Event::DisplayMoved(*(data as *mut DisplayID)) },
                "chunkwm_export_display_resized" => unsafe {
                    Event::DisplayResized(*(data as *mut DisplayID))
                },
                "chunkwm_export_display_changed" => Event::DisplayChanged,
                "chunkwm_export_space_changed" => Event::SpaceChanged,
                "chunkwm_daemon_command" => Event::DaemonCommand((data as PayloadRef).into()),
                _ => Event::Other(event),
            };

            unsafe {
                if let Some(ref mut plugin) = PLUGIN {
                    match plugin.handle(event) {
                        Ok(_) => true,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            false
                        }
                    }
                } else {
                    eprintln!("Error: plugin was not initialised.");
                    return false;
                }
            }
        }

        pub extern "C" fn chunkwm_plugin_init(api: API) -> bool {
            unsafe {
                PLUGIN = Some($struct_ident::new(api));
            }
            true
        }

        pub extern "C" fn chunkwm_plugin_deinit() {
            unsafe {
                if let Some(ref plugin) = PLUGIN {
                    plugin.shutdown();
                }
            }
        }

        pub unsafe extern "C" fn chunkwm_init_plugin_vtable(plugin: *mut ChunkWMPlugin) {
            (*plugin).init = chunkwm_plugin_init;
            (*plugin).deinit = chunkwm_plugin_deinit;
            (*plugin).run = chunkwm_plugin_main;
        }

        pub unsafe extern "C" fn chunkwm_init_plugin_subscriptions(plugin: *mut ChunkWMPlugin) {
            (*plugin).subscriptions = $struct_ident::subscribe().as_ptr();
            (*plugin).subscription_count = $struct_ident::subscribe().len() as c_uint;
        }

        #[no_mangle]
        pub unsafe extern "C" fn get_plugin() -> *mut ChunkWMPlugin {
            use std::sync::{Once, ONCE_INIT};
            static INIT: Once = ONCE_INIT;

            static mut SINGLETON: ChunkWMPlugin = {
                use std::ptr;
                extern "C" fn _init(_api: API) -> bool {
                    false
                }
                extern "C" fn _deinit() {}
                extern "C" fn _run(_node: *const c_char, _data: *mut c_void) -> bool {
                    false
                }
                ChunkWMPlugin {
                    init: _init,
                    deinit: _deinit,
                    run: _run,
                    subscriptions: ptr::null_mut(),
                    subscription_count: 0,
                }
            };

            INIT.call_once(|| {
                chunkwm_init_plugin_vtable(&mut SINGLETON);
                chunkwm_init_plugin_subscriptions(&mut SINGLETON);
            });

            &mut SINGLETON
        }
    }};
}
