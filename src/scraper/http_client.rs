use reqwest::Client;
use std::time::Duration;

use crate::scraper::error::HttpClientError;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Creates a new `HttpClient` with the specified timeout.
    ///
    /// # Errors
    /// Returns `HttpClientError::BuildFailed` if the underlying `reqwest::Client`
    /// cannot be constructed.
    pub fn new(timeout: Duration) -> Result<Self, HttpClientError> {
        let client = Client::builder().timeout(timeout).build()?; // now converts via From
        Ok(Self { client })
    }

    pub fn inner(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use httpmock::Method::GET;

    use super::*;

    #[test]
    fn http_client_new_succeeds() {
        let client = HttpClient::new(Duration::from_secs(1));
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn inner_client_can_make_requests() {
        let server = httpmock::MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200).body("pong");
        });

        let client = HttpClient::new(Duration::from_secs(1)).unwrap();

        let res = client
            .inner()
            .get(server.url("/ping"))
            .send()
            .await
            .unwrap();
        let body = res.text().await.unwrap();

        assert_eq!(body, "pong");
    }
}
