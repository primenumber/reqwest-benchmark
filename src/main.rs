use reqwest::{Client, Result};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = Vec::new();
    for i in 0..16 {
        handles.push(tokio::spawn(async {
            let start = Instant::now();
            let client = Client::new();
            println!("Init: {}ms", start.elapsed().as_millis());
            let res = client.get("http://example.com").send().await.unwrap().text().await.unwrap();
            println!("Downloaded!: {}chars", res.len());
        }));
    }
    for h in handles {
        h.await.unwrap();
    }
    Ok(())
}
