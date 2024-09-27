use std::collections::HashMap;

use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_hosts() {
    mocked(|mock, debrid| async move {
        let r =
            resource::<HashMap<String, models::hosts::Host>>("tests/.resources/hosts/index.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/hosts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.hosts().get().send().await;
        assert!(res.is_ok());
    })
    .await
}
