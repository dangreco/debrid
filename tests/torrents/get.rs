use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_torrents() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<models::torrents::Torrent>>("tests/.resources/torrents/index.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/torrents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.torrents().get().send().await;

        assert!(res.is_ok());
    })
    .await
}
