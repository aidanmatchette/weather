use reqwest;
use serde_json;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let api_key = env::var("WEATHER_API_KEY")
        .unwrap_or_else(|_| "You do not have 'WEATHER_API_KEY' set".to_string());
    let args: Vec<String> = env::args().collect();
    let zip_code = args.get(1).unwrap_or_else(|| {
        println!("Please provide a zip code");
        std::process::exit(1);
    });

    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, zip_code
    );
    let res = client.get(&url).send().await.unwrap();

    if res.status().is_success() {
        let bytes = res.bytes().await.unwrap();

        let json = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap();
        let json_pretty = serde_json::to_string_pretty(&json).unwrap();

        println!("Body: {}", json_pretty);
    } else {
        println!("Request failed with status code: {}", res.status());
    }
}
