struct SubscriptionArray {
    size_t len;
    chunkwm_plugin_export *arr;
};

typedef void * EventHandler;
extern "C" EventHandler chunkwm_rust_create_handler(chunkwm_api *);
extern "C" const char *chunkwm_rust_get_name(void);
extern "C" const char *chunkwm_rust_get_version(void);
extern "C" SubscriptionArray chunkwm_rust_subscribe_to_events();
extern "C" void chunkwm_rust_shutdown_handler(EventHandler);
extern "C" void chunkwm_rust_send_event_with_application(EventHandler, const char *, macos_application);
extern "C" void chunkwm_rust_send_event_with_window(EventHandler, const char *, macos_window);
extern "C" void chunkwm_rust_send_event_with_display(EventHandler, const char *, CGDirectDisplayID);
extern "C" void chunkwm_rust_send_event_with_nothing(EventHandler, const char *);
extern "C" void chunkwm_rust_send_event_with_daemon_command(EventHandler, chunkwm_payload);
