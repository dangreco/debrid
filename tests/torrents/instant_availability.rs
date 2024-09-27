use std::collections::HashMap;

use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_instant_availability() {
    mocked(|mock, debrid| async move {
        let r = resource::<HashMap<String, models::torrents::InstantAvailability>>(
            "tests/.resources/torrents/instant_availability.json",
        );

        Mock::given(matchers::method("GET"))
            .and(matchers::path(
                "/torrents/instantAvailability/c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .torrents()
            .instant_availability()
            .hashes(vec!["c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab".to_string()])
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
