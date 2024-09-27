use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_available_hosts() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<models::torrents::AvailableHost>>(
            "tests/.resources/torrents/available_hosts.json",
        );

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/torrents/availableHosts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.torrents().available_hosts().send().await;

        assert!(res.is_ok());
    })
    .await
}
