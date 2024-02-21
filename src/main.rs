use laralsp::actions::completion::Completion;
use laralsp::actions::snippets::SnippetEngine;
use laralsp::buffer::Buffer;
use laralsp::{DOCUMENT_STATE, PROJECT_CONFIG};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{InputEdit, Point};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, init: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: None,
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![">".to_string()]),
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                    ..Default::default()
                }),

                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, intialized: InitializedParams) {
        self.client
            .log_message(MessageType::LOG, format!("[laralsp] Initialized."))
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        let Ok(mut document_state) = DOCUMENT_STATE.try_lock() else {
            return;
        };
        document_state.insert_state(
            document.uri.clone(),
            Buffer::new(document.text, document.uri).unwrap(),
        )
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let document = params.text_document;

        let Ok(mut documents) = DOCUMENT_STATE.try_lock() else {
            return;
        };

        params
            .content_changes
            .iter()
            .for_each(|change| documents.update_state(&document.uri, change.text.clone()));

        match PROJECT_CONFIG.try_lock() {
            Ok(mut lock) => lock.should_update_config(document.uri),
            Err(_) => {
                return;
            }
        }
    }

    async fn completion(&self, document: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = document.text_document_position.text_document.uri;

        let Ok(buffer) = DOCUMENT_STATE.lock() else {
            return Ok(None);
        };
        let Some(buffer) = buffer.get_state(&uri) else {
            return Ok(None);
        };

        let document_position = document.text_document_position.position;
        let point = Point {
            row: document_position.line as usize,
            column: document_position.character as usize - 1,
        };

        match buffer.complete(point) {
            Ok(completion) if completion.is_some() => Ok(completion),
            _ => Ok(Some(buffer.get_snippets())),
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
