use crate::mocked::*;

use debrid::{models, DebridError, Error};
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_delete_download() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/downloads/delete/ABCDEFGHIJKLMNOP"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid
            .downloads()
            .delete()
            .id("ABCDEFGHIJKLMNOP".to_string())
            .send()
            .await;

        assert!(res.is_ok());
    })
    .await
}

#[tokio::test]
async fn should_fail_to_delete_download() {
    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/downloads"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(models::error::Error {
                    code: 8,
                    message: "bad_token".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.downloads().get().send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/downloads"))
            .respond_with(
                ResponseTemplate::new(403).set_body_json(models::error::Error {
                    code: 14,
                    message: "account_locked".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.downloads().get().send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::AccountLocked))));
    })
    .await;

    mocked(|mock, debrid| async move {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/downloads"))
            .respond_with(
                ResponseTemplate::new(404).set_body_json(models::error::Error {
                    code: 7,
                    message: "unknown_ressource".to_string(),
                }),
            )
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.downloads().get().send().await;

        assert!(res.is_err_and(|e| matches!(e, Error::Debrid(DebridError::ResourceNotFound))));
    })
    .await
}
