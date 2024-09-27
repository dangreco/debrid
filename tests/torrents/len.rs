use crate::mocked::*;

use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_torrents_len() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/torrents"))
            .respond_with(ResponseTemplate::new(200).insert_header("X-Total-Count", "1234"))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.torrents().len().send().await;

        assert!(res.is_ok_and(|n| n == 1234));
    })
    .await
}
