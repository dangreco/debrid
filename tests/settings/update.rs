use crate::mocked::*;

use debrid::{models, DebridError, Error};
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_update_settings() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("POST"))
            .and(matchers::path("/settings/update"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .settings()
            .update()
            .name("locale".to_string())
            .value("fr".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}

#[tokio::test]
async fn should_fail_to_update_settings() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("POST"))
            .and(matchers::path("/settings/update"))
            .respond_with(
                ResponseTemplate::new(400).set_body_json(models::error::Error {
                    code: 1,
                    message: "parameter_missing".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .settings()
            .update()
            .name("foo".to_string())
            .value("bar".to_string())
            .send()
            .await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::MissingParameter))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("POST"))
            .and(matchers::path("/settings/update"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(models::error::Error {
                    code: 8,
                    message: "bad_token".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .settings()
            .update()
            .name("locale".to_string())
            .value("fr".to_string())
            .send()
            .await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("POST"))
            .and(matchers::path("/settings/update"))
            .respond_with(
                ResponseTemplate::new(403).set_body_json(models::error::Error {
                    code: 14,
                    message: "account_locked".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .settings()
            .update()
            .name("locale".to_string())
            .value("fr".to_string())
            .send()
            .await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::AccountLocked))));
    })
    .await;
}
