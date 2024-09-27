use std::{fs, future::Future};

use debrid::Debrid;
use serde::de::DeserializeOwned;
use wiremock::MockServer;

pub async fn mocked<F, Fut, T>(closure: F) -> T
where
    F: FnOnce(MockServer, Debrid) -> Fut,
    Fut: Future<Output = T>,
{
    let mock_server = MockServer::start().await;

    let debrid = Debrid::builder()
        .base_url(mock_server.uri())
        .token("LOREMIPSUM".to_string())
        .build()
        .unwrap();

    closure(mock_server, debrid).await
}

pub fn resource<T: DeserializeOwned>(path: &str) -> T {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap()
}
