//! LPLang Language Server Protocol Implementation

pub mod analysis;
pub mod code_action;
pub mod completion;
pub mod diagnostics;
pub mod goto_def;
pub mod hover;
pub mod rename;
pub mod server;

use anyhow::Result;
use tower_lsp::{LspService, Server};
use std::net::SocketAddr;

/// Start the LSP server.
pub async fn run(addr: SocketAddr) -> Result<()> {
    let (service, socket) = LspService::new(server::Backend::new);
    Server::new(socket, service).serve(addr).await;
    Ok(())
}