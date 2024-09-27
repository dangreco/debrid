use crate::mocked::*;

use debrid::models;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_post_unrestrict_link() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::unrestrict::Link>("tests/.resources/unrestrict/link.json");

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/unrestrict/link"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .unrestrict()
            .link()
            .link("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}