// Browser services (networking, cache, etc.)
use std::error::Error;

pub struct NetworkService {
    client: fantoccini::Client,
}

impl NetworkService {
    pub fn new(client: fantoccini::Client) -> Self {
        NetworkService { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>> {
        self.client.goto(url).await?;
        let html = self.client.source().await?;
        Ok(html)
    }
}
