pub mod identifier_finder;

mod dune_apis;

use crate::dune_apis::*;
use crate::identifier_finder::find_identifier;

use reqwest::Client;
use std::fs::read_to_string;
use tokio;

#[tokio::main]
async fn main() {
    println!("Starting...");
    println!("- Testing find identifier...");
    test_find_identifier();
    let client = reqwest::Client::new();
    println!("- Testing get function name...");
    test_get_function_name(&client).await;
    println!("- Testing get contract name...");
    test_get_contract_name(&client).await;
}

pub fn test_find_identifier() {
    let source_code = read_to_string("examples/erc20.yul").unwrap();
    match find_identifier(&source_code, 22) {
        Ok(Some(identifier)) => match &identifier.location {
            Some(location) => println!("Found '{}' at {}", &identifier, location),
            None => println!("Found '{}'", &identifier),
        },
        Ok(None) => println!("Not found"),
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
