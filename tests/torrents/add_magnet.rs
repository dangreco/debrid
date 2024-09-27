use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_post_add_magnet() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::torrents::AddedTorrent>(
            "tests/.resources/torrents/added_torrent.json",
        );

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/torrents/addMagnet"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .torrents()
            .add_magnet()
            .magnet("LOREMIPSUM".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
