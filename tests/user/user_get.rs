use crate::mocked::*;

use anyhow::Result;
use debrid::{models, Debrid, DebridError, Error};
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_user() {
    mocked(|mock, debrid| async move {
        let r = resource::<models::user::User>("tests/.resources/user/index.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.user().get().send().await;
        assert!(res.is_ok());
    })
    .await
}

#[tokio::test]
async fn should_fail_to_get_user() -> Result<()> {
    let debrid = Debrid::builder().token("LOREMIPSUM".to_string()).build()?;

    let user = debrid.user().get().send().await;

    assert!(user.is_err_and(|e| matches!(e, Error::Debrid(DebridError::BadToken))));

    Ok(())
}
