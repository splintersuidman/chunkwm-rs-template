#include <stdlib.h>
#include <string.h>

#include "../chunkwm/src/api/plugin_api.h"
#include "../chunkwm/src/common/accessibility/application.h"
#include "../chunkwm/src/common/accessibility/window.h"

#include "./rust.h"

#define internal static

internal EventHandler handler;

internal const char *PluginName = chunkwm_rust_get_name();
internal const char *PluginVersion = chunkwm_rust_get_version();
internal chunkwm_api API;

inline bool
StringsAreEqual(const char *A, const char *B)
{
    bool Result = (strcmp(A, B) == 0);
    return Result;
}

/*
 * NOTE(koekeishiya):
 * parameter: const char *Node
 * parameter: void *Data
 * return: bool
 */
PLUGIN_MAIN_FUNC(PluginMain)
{
    return chunkwm_rust_send_event(handler, Node, Data);
}

/*
 * NOTE(koekeishiya):
 * parameter: chunkwm_api ChunkwmAPI
 * return: bool -> true if startup succeeded
 */
PLUGIN_BOOL_FUNC(PluginInit)
{
    API = ChunkwmAPI;
    handler = chunkwm_rust_create_handler(&API);
    return true;
}

PLUGIN_VOID_FUNC(PluginDeInit)
{
    chunkwm_rust_shutdown_handler(handler);
}

// NOTE(koekeishiya): Enable to manually trigger ABI mismatch
#if 0
#undef CHUNKWM_PLUGIN_API_VERSION
#define CHUNKWM_PLUGIN_API_VERSION 0
#endif

// NOTE(koekeishiya): Initialize plugin function pointers.
CHUNKWM_PLUGIN_VTABLE(PluginInit, PluginDeInit, PluginMain)

// NOTE(koekeishiya): Subscribe to ChunkWM events!
SubscriptionArray Subscriptions = chunkwm_rust_subscribe_to_events();
void InitPluginSubscriptions(plugin *Plugin)
{
    Plugin->SubscriptionCount = Subscriptions.len;
    Plugin->Subscriptions = Subscriptions.arr;
}

// NOTE(koekeishiya): Generate plugin
CHUNKWM_PLUGIN(PluginName, PluginVersion);
