mod hacker_news;

use hacker_news::HackerNewsClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = HackerNewsClient::new();

    let max_item = client.max_item().await?;

    println!("The max item is: {max_item}");

    Ok(())
}
