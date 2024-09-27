use crate::mocked::*;

use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_post_unrestrict_folder() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/unrestrict/folder.json");

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/unrestrict/folder"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .unrestrict()
            .folder()
            .link("https://example.com/folder".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}
