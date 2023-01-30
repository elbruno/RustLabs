use reqwest;
use std::collections::HashMap;

// tokio let's us use "async" on our main function
#[tokio::main]
//async fn main() {
    // let response = reqwest::get(" https://v2.jokeapi.dev/joke/Any")        
    // .await
    // .unwrap()
    // .text()
    // .await;
    // println!("{:?}", response);
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Build the client using the builder pattern
        let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://httpbin.org/ip")
        .send()
        .await?;

    // Parse the response body as Json in this case
    let ip = res
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", ip);
    Ok(())
}