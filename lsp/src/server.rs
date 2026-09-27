use tower_lsp::{LspService, Server};

pub mod server {
    use tower_lsp::{LanguageServer, LspService, Server};
    use lsp_types::*;

    pub struct Backend;

    impl Backend {
        pub fn new() -> Self {
            Self
        }
    }

    #[tower_lsp::async_trait]
    impl LanguageServer for Backend {
        async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                    completion_provider: Some(CompletionOptions::default()),
                    hover_provider: Some(HoverProviderCapability::Simple(true)),
                    definition_provider: Some(OneOf::Left(true)),
                    references_provider: Some(OneOf::Left(true)),
                    rename_provider: Some(OneOf::Left(RenameOptions::default())),
                    ..Default::default()
                },
                ..Default::default()
            })
        }

        async fn shutdown(&self) -> Result<(), tower_lsp::jsonrpc::Error> {
            Ok(())
        }

        async fn did_open(&self, _params: DidOpenTextDocumentParams) {}
        async fn did_change(&self, _params: DidChangeTextDocumentParams) {}
        async fn did_save(&self, _params: DidSaveTextDocumentParams) {}
        async fn did_close(&self, _params: DidCloseTextDocumentParams) {}
    }
}