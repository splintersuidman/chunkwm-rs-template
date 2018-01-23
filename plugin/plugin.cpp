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
    if (StringsAreEqual(Node, "chunkwm_export_application_launched") ||
        StringsAreEqual(Node, "chunkwm_export_application_terminated") ||
        StringsAreEqual(Node, "chunkwm_export_application_activated") ||
        StringsAreEqual(Node, "chunkwm_export_application_deactivated") ||
        StringsAreEqual(Node, "chunkwm_export_application_hidden") ||
        StringsAreEqual(Node, "chunkwm_export_application_unhidden"))
    {
        macos_application *application = (macos_application *) Data;
        chunkwm_rust_send_event_with_application(handler, Node, *application);
        return true;
    }
    else if (StringsAreEqual(Node, "chunkwm_export_window_created") ||
        StringsAreEqual(Node, "chunkwm_export_window_destroyed") ||
        StringsAreEqual(Node, "chunkwm_export_window_focused") ||
        StringsAreEqual(Node, "chunkwm_export_window_moved") ||
        StringsAreEqual(Node, "chunkwm_export_window_resized") ||
        StringsAreEqual(Node, "chunkwm_export_window_minimized") ||
        StringsAreEqual(Node, "chunkwm_export_window_deminimized") ||
        StringsAreEqual(Node, "chunkwm_export_window_title_changed"))
    {
        macos_window *window = (macos_window *) Data;
        chunkwm_rust_send_event_with_window(handler, Node, *window);
        return true;
    }
    else if (StringsAreEqual(Node, "chunkwm_export_display_added") ||
        StringsAreEqual(Node, "chunkwm_export_display_removed") ||
        StringsAreEqual(Node, "chunkwm_export_display_moved") ||
        StringsAreEqual(Node, "chunkwm_export_display_resized"))
    {
        CGDirectDisplayID *display = (CGDirectDisplayID *) Data;
        chunkwm_rust_send_event_with_display(handler, Node, *display);
    }
    else if (StringsAreEqual(Node, "chunkwm_daemon_command"))
    {
        chunkwm_payload *payload = (chunkwm_payload *) Data;
        chunkwm_rust_send_event_with_daemon_command(handler, *payload);
    }
    else
    {
        chunkwm_rust_send_event_with_nothing(handler, Node);
    }

    return false;
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
