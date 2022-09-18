use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::{Result, ErrorCode, Error};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub document_map: DashMap<String, Rope>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {

        let uri = params.text_document_position_params.text_document.uri.clone();
        let rope = self.document_map.get(&uri.to_string()).unwrap();

        let position = params.text_document_position_params.position;
        let byte_start = rope.try_line_to_byte(position.line as usize).unwrap();
        let byte_end = rope.try_char_to_byte(position.character as usize).unwrap();
        let byte_offset = byte_start + byte_end as usize;
        let source = rope.to_string();

        /*match find_definition(source, byte_offset) {


        }
        */

        let start_position = Position::new(9 as u32, 3 as u32);
        let end_position = Position::new(9 as u32, 9 as u32);
        let range = Range::new(start_position, end_position);

        let definition = Some(GotoDefinitionResponse::Scalar(Location::new(uri, range)));

        return Ok(definition)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;

        self.document_map.remove(&params.text_document.uri.to_string());
    }
}

pub struct TextDocumentItem {
    uri: Url,
    text: String,
    version: i32,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());
    }
}
