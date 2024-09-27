use debrid::{models, DebridError, Error};
use wiremock::{matchers, Mock, ResponseTemplate};

use crate::mocked::mocked;

#[tokio::test]
async fn should_disable_access_token() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/disable_access_token"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.disable_access_token().send().await;
        assert!(res.is_ok());
    })
    .await
}

#[tokio::test]
async fn should_fail_to_disable_access_token() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/disable_access_token"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(models::error::Error {
                    code: 8,
                    message: "bad_token".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.disable_access_token().send().await;
        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));
    })
    .await
}
