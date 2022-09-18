use crate::definition_finder::find_definition;
use crate::dune_apis::{QUERY_FUNCTION_NAME_SIGNATURE, get_function_name};
use crate::literal_finder::{find_literal, LiteralKind};
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::{Error, ErrorCode, Result};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use yultsur::yul_parser::parse_block;

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
                hover_provider: Some(HoverProviderCapability::Simple(true)),
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

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        self.client.log_message(MessageType::INFO, "hover").await;

        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let rope = self.document_map.get(&uri.to_string()).unwrap();

        let position = params.text_document_position_params.position;
        let byte_start = rope.try_line_to_byte(position.line as usize).unwrap();
        let byte_end = rope.try_char_to_byte(position.character as usize).unwrap();
        let byte_offset = byte_start + byte_end as usize;
        let source = rope.to_string();
        match parse_block(&source) {
            Err(_) => Err(Error::new(ErrorCode::ParseError)),
            Ok(ast) => {
                if let Some(literal) = find_literal(&ast, byte_offset, LiteralKind::Selector) {
                    self.client
                        .log_message(MessageType::ERROR, "Hovering over a selector. Querying Dune API.")
                        .await;

                    let reqwest_client = reqwest::Client::new();
                    match get_function_name(&reqwest_client, QUERY_FUNCTION_NAME_SIGNATURE, literal.literal).await {
                        Err(message) => {
                            self.client
                                .log_message(MessageType::ERROR, format!("Server error: {}", message))
                                .await;
                            Err(Error::new(ErrorCode::ServerError(1)))
                        }
                        Ok(signature) => {
                            let tooltip = format!("**Signature**: {}", signature);

                            Ok(Some(Hover {
                                contents: HoverContents::Scalar(MarkedString::String(tooltip.to_string())),
                                range: None,
                            }))
                        }
                    }
                } else if let Some(_literal) = find_literal(&ast, byte_offset, LiteralKind::Address)
                {
                    let tooltip = "address (**TODO**: Dune query)";
                    Ok(Some(Hover {
                        contents: HoverContents::Scalar(MarkedString::String(tooltip.to_string())),
                        range: None,
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let rope = self.document_map.get(&uri.to_string()).unwrap();

        let position = params.text_document_position_params.position;
        let byte_start = rope.try_line_to_byte(position.line as usize).unwrap();
        let byte_end = rope.try_char_to_byte(position.character as usize).unwrap();
        let byte_offset = byte_start + byte_end as usize;
        let source = rope.to_string();

        match find_definition(&source, byte_offset) {
            Ok(Some(identifier)) => match identifier.location {
                None => Ok(None),
                Some(location) => {
                    let start_line = rope.try_byte_to_line(location.start).unwrap();
                    let end_line = rope.try_byte_to_line(location.end).unwrap();
                    let start_index =
                        location.start as u32 - rope.try_line_to_byte(start_line).unwrap() as u32;
                    let end_index =
                        location.end as u32 - rope.try_line_to_byte(end_line).unwrap() as u32;
                    let start_char = end_index - start_index;
                    let end_char = start_char;
                    let range = Range::new(
                        Position::new(start_line as u32, start_char),
                        Position::new(end_line as u32, end_char),
                    );

                    let msg = format!("byte offset: {:?}\nrange: {:?}", byte_offset, range);
                    self.client.log_message(MessageType::INFO, msg).await;

                    Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
                        uri, range,
                    ))))
                }
            },
            Ok(None) => Ok(None),
            Err(_) => return Err(Error::new(ErrorCode::InvalidParams)),
        }
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

        self.document_map
            .remove(&params.text_document.uri.to_string());
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
