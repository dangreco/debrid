use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_torrent_info() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::torrents::TorrentInfo>("tests/.resources/torrents/info.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/torrents/info/ABCDEFGHIJKLM"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .torrents()
            .info()
            .id("ABCDEFGHIJKLM".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
