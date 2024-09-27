use wiremock::{matchers, Mock, ResponseTemplate};

use crate::mocked::mocked;

#[tokio::test]
async fn should_get_time_iso() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/time/iso"))
            .respond_with(ResponseTemplate::new(200).set_body_string("2024-09-27T21:53:04+0200"))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.time_iso().send().await;
        assert!(res.is_ok());
    })
    .await
}
