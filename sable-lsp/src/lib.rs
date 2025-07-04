use std::sync::{Arc, Mutex};

use bumpalo::Bump;
use ariadne::Report;
use sable_ast::ast::Ast;
use sable_common::{cache::AriadneCache, manager::Manager, source::Source, writer::Sink};
use sable_parser::{lexer::Lexer, parser::Parser as SableParser};
use sable_errors::parse_error::ParseError;

use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

pub struct Backend {
    client: Client,
    state: Arc<Mutex<SharedState>>,
}

pub struct SharedState {
    pub cache: AriadneCache,
    pub manager: Manager<'static>,
    pub bump: Box<Bump>,
}

struct NoopSink;

impl Sink for NoopSink {
    type Error = ();

    fn report(&mut self, _report: ariadne::Report<'_, sable_common::FileSpan>) -> std::result::Result<(), Self::Error> {
        Ok(())
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self { cache: AriadneCache::new(), manager: Manager::<'static>::new(), bump: Box::new(Bump::new()) }
    }
}

impl Backend {
    pub fn new(client: Client, state: Arc<Mutex<SharedState>>) -> Self {
        Self { client, state }
    }

    fn offset_to_position(source: &Source<'static>, offset: usize) -> Position {
        let text = &source.content()[..offset.min(source.content().len())];
        let mut line = 0u32;
        let mut col = 0u32;
        for l in text.lines() {
            line += 1;
            col = l.chars().count() as u32;
        }
        if !text.ends_with('\n') && line > 0 {
            line -= 1;
        }
        Position::new(line, col)
    }

    fn diagnostic_from_error(source: &Source<'static>, err: &ParseError<'static>) -> Diagnostic {
        let (range, message) = match err {
            ParseError::UnexpectedToken(e) => {
                let loc = e.found().location();
                (loc.range().clone(), format!("Unexpected token: {:?}", e.found().kind()))
            }
            ParseError::UnknownChar(e) => {
                (e.location.range().clone(), format!("Unknown character: `{}`", e.lexeme))
            }
            ParseError::NumericError(e) => {
                (e.location.range().clone(), format!("Invalid number: `{}`", e.lexeme))
            }
        };

        let start = Self::offset_to_position(source, range.start);
        let end = Self::offset_to_position(source, range.end);

        Diagnostic {
            range: Range { start, end },
            severity: Some(DiagnosticSeverity::ERROR),
            message,
            ..Default::default()
        }
    }

    async fn parse_and_publish(&self, uri: Url, text: String) {
        let (diagnostics, _ast) = {
            let mut state = self.state.lock().unwrap();
            let bump: &'static Bump = unsafe { &*(&*state.bump as *const Bump) };
            let filename = bump.alloc_str(uri.path());
            let source = state.manager.add_source(&text, filename, bump);
            state.cache.add_file(&source);

            let lexer = Lexer::new(source.clone());
            let mut ast = Ast::new(bump);
            let mut parser = SableParser::new(lexer, &mut ast);
            let mut sink = NoopSink;
            let mut errors = Vec::new();
            let status = parser.parse_collect(&mut sink, &mut errors);

            let diagnostics: Vec<Diagnostic> = errors
                .iter()
                .map(|e| Self::diagnostic_from_error(&source, e))
                .collect();

            (diagnostics, status)
        };

        self.client.publish_diagnostics(uri, diagnostics, None).await;
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
