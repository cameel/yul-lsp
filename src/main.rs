use reqwest;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::{thread, time};
use tokio;

static QUERY_FUNCTION_NAME_SIGNATURE: i32 = 1279121;
static DUNE_API_KEY: &str = "<insert key here>";

#[tokio::main]
async fn main() {
    println!("Starting...");

    let client = reqwest::Client::new();

    let function_signature = "0x70a08231".to_owned();

    println!("Function signature: {:?}", function_signature);

    let function_name =
        get_function_name(&client, QUERY_FUNCTION_NAME_SIGNATURE, function_signature).await;

    //let result = get_function_name(client);
    println!("Function name {:?}", function_name);
}

async fn get_function_name(client: &Client, query_id: i32, function_signature: String) -> String {
    // TODO: (fix) figure out better way than "replace"
    let body = r#"{
        "query_parameters": {
            "query_id":"function_signature"
        }
      }"#
    .replace("function_signature", &function_signature);

    // Execute query
    let query_execution = execute_query(&client, query_id, body).await;

    let query_execution_object: Value = match serde_json::from_str(query_execution.as_str()) {
        Ok(it) => it,
        Err(_) => panic!("[Error]"),
    };

    let execution_id = &query_execution_object["execution_id"];

    // Wait 3 sec till the query fetching completed
    thread::sleep(time::Duration::from_secs(3));

    // Get query results
    let query_results = get_query_results(&client, execution_id).await;

    let query_results_object: Value = match serde_json::from_str(query_results.as_str()) {
        Ok(it) => it,
        Err(_) => panic!("[Error]"),
    };

    let signature_name = query_results_object["result"]["rows"][0]["signature"].to_string();

    signature_name
}

async fn execute_query(client: &Client, query_id: i32, body: String) -> String {
    let query_url = format_query_url(query_id);

    let execution_result = client
        .post(query_url)
        .header("x-dune-api-key", DUNE_API_KEY.clone())
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    execution_result
}

async fn get_query_results(client: &Client, execution_id: &Value) -> String {
    let execute_url = format_exection_url(execution_id);

    let execution_result = client
        .get(execute_url)
        .header("x-dune-api-key", DUNE_API_KEY.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    execution_result
}

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
