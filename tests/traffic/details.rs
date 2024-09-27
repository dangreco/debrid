use std::collections::HashMap;

use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_traffic_details() {
    mocked(|mock, debrid| async move {
        let r = resource::<HashMap<String, models::traffic::Detail>>(
            "tests/.resources/traffic/details.json",
        );

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/traffic/details"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.traffic().details().send().await;

        assert!(res.is_ok());
    })
    .await
}
