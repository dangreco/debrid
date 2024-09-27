use crate::mocked::*;

use debrid::models;
use tokio::fs::File;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_put_add_torrent() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::torrents::AddedTorrent>(
            "tests/.resources/torrents/added_torrent.json",
        );

        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/torrents/addTorrent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();

        let res = debrid.torrents().add_torrent().file(file).send().await;

        assert!(res.is_ok());
    })
    .await
}
