use std::sync::{Arc, Mutex};

use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};
use sable_common::{cache::AriadneCache, manager::Manager};

pub struct Backend {
    client: Client,
    state: Arc<Mutex<SharedState>>,
}

pub struct SharedState {
    pub cache: AriadneCache,
    pub manager: Manager<'static>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self { cache: AriadneCache::new(), manager: Manager::<'static>::new() }
    }
}

impl Backend {
    pub fn new(client: Client, state: Arc<Mutex<SharedState>>) -> Self {
        Self { client, state }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Sable LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {}

    async fn did_change(&self, _: DidChangeTextDocumentParams) {}
}

pub async fn run_server() {
    let state = Arc::new(Mutex::new(SharedState::default()));
    let (service, socket) = LspService::new(|client| Backend::new(client, state.clone()));
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
