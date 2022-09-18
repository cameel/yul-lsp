pub mod definition_finder;
pub mod identifier_finder;
pub mod literal_finder;

mod dune_apis;
mod lsp_server;

use crate::definition_finder::find_definition;
use crate::dune_apis::*;
use crate::identifier_finder::find_identifier;
use yultsur::dialect::EVMDialect;
use yultsur::resolver::resolve;
use yultsur::yul::IdentifierID;
use yultsur::yul_parser::parse_block;

use crate::lsp_server::Backend;
use dashmap::DashMap;
use tower_lsp::{LspService, Server};

use reqwest::Client;
use std::fs::read_to_string;
use tokio;

#[tokio::main]
async fn main() {
    env_logger::init();

    /*println!("Starting...");
    println!("- Testing find identifier...");
    test_find_identifier(100);
    test_find_definition(100);
    let client = reqwest::Client::new();
    println!("- Testing get function name...");
    test_get_function_name(&client).await;
    println!("- Testing get contract name...");
    test_get_contract_name(&client).await;*/

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

pub async fn test_get_function_name(client: &Client) {
    let function_signature = "0x70a08231".to_owned();
    println!("Function signature: {}", function_signature);
    let function_name =
        get_function_name(&client, QUERY_FUNCTION_NAME_SIGNATURE, function_signature).await;
    println!("Function name:      {}", function_name);
}

pub async fn test_get_contract_name(client: &Client) {
    let contract_address = "0xe592427a0aece92de3edee1f18e0157c05861564".to_owned();
    println!("Contract Address:{}", contract_address);
    let contract_name = get_contract_name(&client, QUERY_CONTRACT_NAME, contract_address).await;
    println!("Contract Name:   {}", contract_name);
}
