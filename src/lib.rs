mod dotenv;
mod env_type;
mod logger;
mod natives;
mod plugin;
mod store;

use plugin::EnvPlugin;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [EnvPlugin::env, EnvPlugin::env_count],
    {
        return EnvPlugin::new();
    }
);
