use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_post_unrestrict_check() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::unrestrict::Check>("tests/.resources/unrestrict/check.json");

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/unrestrict/check"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .unrestrict()
            .check()
            .link("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
