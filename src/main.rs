//! Process entry point for the `nu_plugin_net` Nushell plugin binary.

use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_net::Plugin;

fn main() {
    serve_plugin(&Plugin::new(), MsgPackSerializer);
}
