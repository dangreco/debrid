use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_active_count() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::torrents::ActiveCount>(
            "tests/.resources/torrents/active_count.json",
        );

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/torrents/activeCount"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.torrents().active_count().send().await;

        assert!(res.is_ok());
    })
    .await
}
