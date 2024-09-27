use crate::mocked::*;

use debrid::{models, DebridError, Error};
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_settings() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::settings::Settings>("tests/.resources/settings/index.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.settings().get().send().await;

        assert!(res.is_ok());
    })
    .await
}

#[tokio::test]
async fn should_fail_to_get_settings() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/settings"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(models::error::Error {
                    code: 8,
                    message: "bad_token".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.settings().get().send().await;
        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));
    })
    .await;
   
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/settings"))
            .respond_with(
                ResponseTemplate::new(403).set_body_json(models::error::Error {
                    code: 14,
                    message: "account_locked".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.settings().get().send().await;
        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::AccountLocked))));
    })
    .await
}
