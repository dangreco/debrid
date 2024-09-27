use crate::mocked::*;

use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_post_unrestrict_container_link() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/unrestrict/container_link.json");

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/unrestrict/containerLink"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .unrestrict()
            .container_link()
            .link("https://example.com/folder".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
