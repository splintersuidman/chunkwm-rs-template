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
extern "C" bool chunkwm_rust_send_event(EventHandler, const char *, void *);
