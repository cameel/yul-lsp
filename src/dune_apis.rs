extern crate dotenv;

use lazy_static::lazy_static;
use reqwest::Client;
use reqwest::{self};
use serde_json::Value;
use std::fs::read_to_string;
use std::{thread, time};

pub static QUERY_FUNCTION_NAME_SIGNATURE: i32 = 1279121;
pub static QUERY_CONTRACT_NAME: i32 = 1279874;

lazy_static! {
    static ref DUNE_API_KEY: String = get_dune_api_key().unwrap();
}

pub async fn get_function_name(
    client: &Client,
    query_id: i32,
    function_signature: String,
) -> String {
    // TODO: (fix) figure out better way than "replace"
    let body = r#"{
        "query_parameters": {
            "query_id":"function_signature"
        }
      }"#
    .replace("function_signature", &function_signature);

    // Execute query
    let query_execution = match execute_query(&client, query_id, body).await {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let query_execution_object: Value = match serde_json::from_str(query_execution.as_str()) {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let execution_id = &query_execution_object["execution_id"];

    // Wait 3 sec till the query fetching completed
    thread::sleep(time::Duration::from_secs(5));

    // Get query results
    let query_results = match get_query_results_text(&client, execution_id).await {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let query_results_object: Value = match serde_json::from_str(query_results.as_str()) {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let signature_name = query_results_object["result"]["rows"][0]["signature"].to_string();

    signature_name
}

pub async fn get_contract_name(client: &Client, query_id: i32, contract_address: String) -> String {
    // TODO: (fix) figure out better way than "replace"
    let body = r#"{
        "query_parameters": {
            "contract_address":"contract_address_string"
        }
      }"#
    .replace("contract_address_string", &contract_address);

    // Execute query
    let query_execution = match execute_query(&client, query_id, body).await {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let query_execution_object: Value = match serde_json::from_str(query_execution.as_str()) {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let execution_id = &query_execution_object["execution_id"];

    // Wait 3 sec till the query fetching completed
    thread::sleep(time::Duration::from_secs(5));

    // Get query results
    let query_results = match get_query_results_text(&client, execution_id).await {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let query_results_object: Value = match serde_json::from_str(query_results.as_str()) {
        Ok(it) => it,
        Err(error) => return format!("{}", error),
    };

    let contract_name = query_results_object["result"]["rows"][0]["name"].to_string();

    contract_name
}

async fn execute_query(
    client: &Client,
    query_id: i32,
    body: String,
) -> Result<String, reqwest::Error> {
    let query_url = format_query_url(query_id);

    let execution_result = client
        .post(query_url)
        .header("x-dune-api-key", DUNE_API_KEY.clone())
        .body(body)
        .send()
        .await?
        .text()
        .await?;

    Ok(execution_result)
}

async fn get_query_results_text(
    client: &Client,
    execution_id: &Value,
) -> Result<String, reqwest::Error> {
    let execute_url = format_exection_url(execution_id);

    let execution_result = client
        .get(execute_url)
        .header("x-dune-api-key", DUNE_API_KEY.clone())
        .send()
        .await?
        .text()
        .await?;

    Ok(execution_result)
}

// async fn get_query_results_json(client: &Client, execution_id: &Value) -> Result<Value, reqwest::Error> {
//     let execute_url = format_exection_url(execution_id);
//     let execution_result = client
//         .get(execute_url)
//         .header("x-dune-api-key", DUNE_API_KEY.clone())
//         .send()
//         .await?
//         .json::<serde_json::Value>()
//         .await?;
//     Ok(execution_result["result"]["rows"][0]["signature"].clone())
// }

fn format_query_url(query_id: i32) -> String {
    let query_url = format!("https://api.dune.com/api/v1/query/{}/execute", query_id);

    query_url
}

fn format_exection_url(execution_id: &Value) -> String {
    // TODO: fix the replace
    let execute_url = format!(
        "https://api.dune.com/api/v1/execution/{}/results",
        execution_id
    )
    .replace('\"', "");

    execute_url
}

fn get_dune_api_key() -> Result<String, std::io::Error> {
    let env_file = read_to_string(".env")
        .unwrap()
        .replace("DUNE_API_KEY=", "")
        .replace("\"", "");

    Ok(env_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_function_name() {
        let client = reqwest::Client::new();
        let function_signature = "0x70a08231".to_owned();
        println!("Function signature: {}", function_signature);
        let function_name =
            get_function_name(&client, QUERY_FUNCTION_NAME_SIGNATURE, function_signature).await;
        println!("Function name:      {}", function_name);

        assert_eq!(function_name, "balanceOf(address)");
    }

    #[test]
    fn test_get_contract_name() {
        let client = reqwest::Client::new();
        let contract_address = "0xe592427a0aece92de3edee1f18e0157c05861564".to_owned();
        println!("Contract Address:{}", contract_address);
        let contract_name = get_contract_name(&client, QUERY_CONTRACT_NAME, contract_address).await;
        println!("Contract Name:   {}", contract_name);

        assert_eq!(contract_name, "SwapRouter");
    }
}
