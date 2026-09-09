use crate::InterfacesCommand;

/// The plugin entry point that registers the `net` command with Nushell.
pub struct Plugin;

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin {
    /// Construct a plugin instance ready to be served by Nushell.
    pub fn new() -> Plugin {
        Plugin {}
    }
}

impl nu_plugin::Plugin for Plugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(InterfacesCommand)]
    }
}
