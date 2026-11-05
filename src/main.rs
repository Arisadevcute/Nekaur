mod aurdecctl;

use reqwest;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("my-aur-client/0.1")
        .build()?;

    let response = client
        .get("https://aur.archlinux.org/rpc/v5/search/google-chrome")
        .send()
        .await?;

    println!("status = {}", response.status());
    println!("content-type = {:?}", response.headers().get("content-type"));

    let body = response.text().await?;
    println!("body = {}", &body[..body.len().min(500)]);

    let resp: Value = serde_json::from_str(&body)?;

    for pkg in resp["results"].as_array().unwrap() {
        println!("{} {}", pkg["Name"], pkg["Version"]);
    }

    Ok(())
}