//! Dynamic (subprocess) entrypoint for the openvpn plugin.
//!
//! Serves this plugin over the orca socket via the typed `Plugin` builder.
//! The plugin is a `[[bin]]`, owns no runtime, and reaches orca only through
//! the socket. Advertises a single `service` backend.
plugin_toolkit::instrument::bootstrap!();
use openvpn::OpenvpnBackend;
use plugin_toolkit::plugin::Plugin;

fn main() -> plugin_toolkit::anyhow::Result<()> {
    Plugin::named("openvpn")
        .version(env!("CARGO_PKG_VERSION"))
        .service(OpenvpnBackend::new("openvpn"))
        .serve()
}
