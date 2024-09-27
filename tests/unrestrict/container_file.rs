use crate::mocked::*;

use tokio::fs::File;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_put_unrestrict_container_file() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/unrestrict/container_file.json");

        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/unrestrict/containerFile"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();

        let res = debrid.unrestrict().container_file().file(file).send().await;

        assert!(res.is_ok());
    })
    .await
}
