use reqwest::{Client, Result};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let client = Client::new();
    println!("Init: {}ms", start.elapsed().as_millis());
    let res = client.get("https://example.com").send().await?.text().await?;
    println!("{}", res);
    Ok(())
}
