#include <stdlib.h>
#include <string.h>

#include "../chunkwm/src/api/plugin_api.h"
#include "../chunkwm/src/common/accessibility/application.h"
#include "../chunkwm/src/common/accessibility/window.h"

extern "C" const char *chunkwm_rust_get_name(void);
extern "C" const char *chunkwm_rust_get_version(void);

// extern "C" bool chunkwm_plugin_main(const char *Node, void *Data);
// extern "C" bool chunkwm_plugin_init(chunkwm_api ChunkwmAPI);
// extern "C" void chunkwm_plugin_deinit(void);
// extern "C" void init_plugin_vtable(plugin *Plugin);
// extern "C" void init_plugin_subscriptions(plugin *Plugin);
extern "C" plugin *get_plugin();

#define internal static

internal const char *PluginName = chunkwm_rust_get_name();
internal const char *PluginVersion = chunkwm_rust_get_version();

CHUNKWM_EXTERN
{
    plugin *GetPlugin()
    {
        return get_plugin();
    }
    plugin_details Exports =
    {
        CHUNKWM_PLUGIN_API_VERSION,
        __FILE__,
        PluginName,
        PluginVersion,
        get_plugin,
    };
}
