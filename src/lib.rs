//! A Nushell plugin for inspecting the network interfaces on the current host.
//!
//! The plugin exposes the [`InterfacesCommand`] command under the Nushell name
//! `net`. Each invocation returns the interfaces reported by `pnet`, including
//! their addresses, operating-system index, optional MAC address, and common
//! interface flags.

mod inf;
mod plugin;

pub use inf::InterfacesCommand;
pub use plugin::Plugin;
