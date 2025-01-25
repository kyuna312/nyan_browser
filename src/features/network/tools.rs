use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct NetworkTools;

impl NetworkTools {
    pub async fn modify_headers(
        &mut self,
        headers: HeaderModifications,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn mock_responses(&mut self, mocks: Vec<MockResponse>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderModifications {
    pub add: Vec<(String, String)>,
    pub remove: Vec<String>,
    pub modify: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockResponse {
    pub url_pattern: String,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
