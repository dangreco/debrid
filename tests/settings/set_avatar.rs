use crate::mocked::*;

use debrid::{models, DebridError, Error};
use tokio::fs::File;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_set_avatar() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/settings/avatarFile"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();
        let res = debrid.settings().set_avatar().file(file).send().await;
        assert!(res.is_ok());
    })
    .await;
}

#[tokio::test]
async fn should_fail_to_set_avatar() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/settings/avatarFile"))
            .respond_with(
                ResponseTemplate::new(400).set_body_json(models::error::Error {
                    code: 27,
                    message: "upload_error".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();
        let res = debrid.settings().set_avatar().file(file).send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::UploadError))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/settings/avatarFile"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(models::error::Error {
                    code: 8,
                    message: "bad_token".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();
        let res = debrid.settings().set_avatar().file(file).send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("PUT"))
            .and(matchers::path("/settings/avatarFile"))
            .respond_with(
                ResponseTemplate::new(403).set_body_json(models::error::Error {
                    code: 14,
                    message: "account_locked".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let file = File::open("Cargo.toml").await.unwrap();
        let res = debrid.settings().set_avatar().file(file).send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::AccountLocked))));
    })
    .await;
}
