use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;


#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // - Create a new client which is re-used between requests
    let client = reqwest::Client::new();

    /// Business logic comes here

    // Create a Map of string key-value pairs 
    // to represent the body payload
    let mut map = HashMap::new();
    map.insert("model", "text-davinci-003");
    map.insert("prompt", "describe rust programming language");
    map.insert("temperature", "0.9");
    map.insert("max_tokens", "1024");


    // - Doing a POST request
    // - Parse the response to the "JSONResponse" struct
    let resp_json = client.post("https://api.openai.com/v1/completions")
        .json(&map)
        .send()
        .await?
        .json::<JSONResponse>()
        .await?;

    println!("{:#?}", resp_json);

    Ok(())
}