#include "../chunkwm/src/api/plugin_api.h"

extern "C" const char *chunkwm_rust_get_name(void);
extern "C" const char *chunkwm_rust_get_version(void);
extern "C" plugin *get_plugin();

static const char *PluginName = chunkwm_rust_get_name();
static const char *PluginVersion = chunkwm_rust_get_version();

extern "C"
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
