use std::collections::HashMap;

use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_traffic() {
    mocked(|mock, debrid| async move {
        let r = resource::<HashMap<String, models::traffic::Traffic>>(
            "tests/.resources/traffic/index.json",
        );

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/traffic"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.traffic().get().send().await;

        assert!(res.is_ok());
    })
    .await
}
