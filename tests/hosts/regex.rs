use crate::mocked::*;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_hosts_regex() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/hosts/regex.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/hosts/regex"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.hosts().regex().send().await;
        assert!(res.is_ok());

        let regexes = res.unwrap();
        assert!(regexes[0].is_match("https://example0.com/abcdefghijkl"));
        assert!(regexes[1].is_match("https://example1.com/abcdefghijkl"));
        assert!(regexes[2].is_match("https://example2.com/abcdefghijkl"));
    })
    .await
}
