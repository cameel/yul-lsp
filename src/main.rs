pub mod definition_finder;
pub mod dune_apis;
pub mod identifier_finder;
pub mod literal_finder;

mod lsp_server;

use crate::definition_finder::find_definition;
use crate::identifier_finder::find_identifier;
use yultsur::dialect::EVMDialect;
use yultsur::resolver::resolve;
use yultsur::yul::IdentifierID;
use yultsur::yul_parser::parse_block;

use crate::lsp_server::Backend;
use dashmap::DashMap;
use tower_lsp::{LspService, Server};

use std::fs::read_to_string;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
    })
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

pub fn test_find_identifier(cursor_position: usize) {
    let source_code = read_to_string("examples/erc20.yul").unwrap();
    match parse_block(&source_code) {
        Ok(mut ast) => {
            resolve::<EVMDialect>(&mut ast);

            match find_identifier(&ast, cursor_position) {
                Some(reference) => {
                    match &reference.location {
                        Some(location) => {
                            println!("Reference to '{}' at {}.", &reference, location)
                        }
                        None => println!("Reference to '{}'.", &reference),
                    };

                    match reference.id {
                        IdentifierID::Declaration(id) => {
                            println!("The identifier is a definition (id: {}).", id)
                        }
                        IdentifierID::Reference(id) => {
                            println!("The identifier is a reference (id: {}).", id)
                        }
                        IdentifierID::BuiltinReference => println!("The identifier is a built-in."),
                        IdentifierID::UnresolvedReference => {
                            println!("The identifier has not been resolved yet.")
                        }
                    };
                }
                None => println!("Not found"),
            };
        }
        Err(error) => println!("{}", error),
    }
}

pub fn test_find_definition(cursor_position: usize) {
    let source_code = read_to_string("examples/erc20.yul").unwrap();
    match find_definition(&source_code, cursor_position) {
        Ok(Some(definition)) => match &definition.location {
            Some(location) => println!("Definition of '{}' found at {}.", &definition, location),
            None => println!("Definition of '{}' found.", &definition),
        },
        Ok(None) => println!("Definition not found"),
        Err(error) => println!("{}", error),
    }
}
