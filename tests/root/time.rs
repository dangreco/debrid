use wiremock::{matchers, Mock, ResponseTemplate};

use crate::mocked::mocked;

#[tokio::test]
async fn should_get_time() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/time"))
            .respond_with(ResponseTemplate::new(200).set_body_string("2024-09-27 21:52:18"))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.time().send().await;
        assert!(res.is_ok());
    })
    .await
}
