use reqwest;
use reqwest::Client;
use std::env;
use tokio;

static QUERY_FUNCTION_NAME_SIGNATURE: i32 = 1279121;
static DUNE_API_KEY: String = match env::var_os("DUNE_API_KEY") {
    Some(v) => v.into_string().unwrap(),
    None => panic!("$DUNE_API_KEY is not set"),
};

#[tokio::main]
async fn main() {
    print!("Starting");

    let client = reqwest::Client::new();

    let query_result = get_function_name(
        client,
        QUERY_FUNCTION_NAME_SIGNATURE,
        "0x70a08231".to_owned(),
    );

    //let result = get_function_name(client);
    println!("----------------- Success! {:?}", query_result);
}

async fn get_function_name(client: Client, query_id: i32, function_signature: String) -> String {
    let body = r#"{
        "query_parameters": {
            "query_id":"{function_signature}"
        }
      }"#
    .to_owned();

    let query_result = execute_query(client, query_id, body);

    println!("----------------- Success 1 ! {:?}", query_result);
    query_result
}

async fn execute_query(client: Client, query_id: i32, body: String) -> Result<String, > {
    let execute_query_url = format_query_url(query_id);

    let execution_result = match client
        .post(execute_query_url)
        .header("x-dune-api-key", DUNE_API_KEY)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
    {
        Ok(it) => it,
        Err(err) => return Ok(Err(err)),
    };

    println!("----------------- Success 2 ! {:?}", execution_result);

    Ok(execution_result)
}

fn format_query_url(query_id: i32) -> String {
    let execute_query_url = format!("https://api.dune.com/api/v1/query/{}/execute", query_id);

    execute_query_url
}
