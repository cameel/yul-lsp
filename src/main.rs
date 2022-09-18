pub mod identifier_finder;

mod dune_apis;

use crate::dune_apis::*;
use crate::identifier_finder::find_identifier;
use yultsur::dialect::EVMDialect;
use yultsur::resolver::resolve;
use yultsur::yul::IdentifierID;
use yultsur::yul_parser::parse_block;

use reqwest::Client;
use std::fs::read_to_string;
use tokio;

#[tokio::main]
async fn main() {
    test_find_identifier(100);

    println!("Starting...");
    println!("- Testing find identifier...");
    test_find_identifier(100);
    let client = reqwest::Client::new();
    println!("- Testing get function name...");
    test_get_function_name(&client).await;
    println!("- Testing get contract name...");
    test_get_contract_name(&client).await;
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
        },
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
