use reqwest::{Client, Error, Url};
use std::str::FromStr;

/// Hacker News API base url.
/// Full documentation: https://github.com/HackerNews/API
const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// HTTP client to make requests to the Hacker News API.
pub struct HackerNewsClient {
    pub client: reqwest::Client,
    base_url: Url,
}

impl HackerNewsClient {
    /// Initialise a new HTTP client
    pub fn new() -> HackerNewsClient {
        let client = Client::new();
        let base_url = Url::from_str(BASE_URL).expect("Invalid base URL");

        HackerNewsClient { client, base_url }
    }

    /// Get the id of the latest item published on Hacker News
    pub async fn max_item(&self) -> Result<usize, Error> {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("maxitem.json");

        let response = self.client.get(url).send().await?;
        let max_item: usize = response.json().await?;

        Ok(max_item)
    }

    /// Get the ids of up to 500 top stories of Hacker News
    pub async fn get_top_stories(&self) {
        todo!()
    }

    /// Get details of an individual Hacker News item
    pub async fn get_item(&self) {
        todo!()
    }

    /// Get details of a user
    pub async fn get_user(&self) {
        todo!()
    }
}
