use crate::mocked::*;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_hosts_domains() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/hosts/domains.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/hosts/domains"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.hosts().domains().send().await;
        assert!(res.is_ok());
    })
    .await
}
