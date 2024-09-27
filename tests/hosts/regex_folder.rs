use crate::mocked::*;
use wiremock::{matchers, Mock, ResponseTemplate};

#[tokio::test]
async fn should_get_hosts_regex_folder() {
    mocked(|mock, debrid| async move {
        let r = resource::<Vec<String>>("tests/.resources/hosts/regex_folder.json");

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/hosts/regexFolder"))
            .respond_with(ResponseTemplate::new(200).set_body_json(r))
            .expect(1)
            .mount(&mock)
            .await;

        let res = debrid.hosts().regex_folder().send().await;
        assert!(res.is_ok());

        let regexes = res.unwrap();

        assert!(regexes[0].is_match(
            "https://docs.google.com/drive/folders/123ABC456def789ghi_jklMNOpqrsTUVwxyz"
        ));
        assert!(regexes[0].is_match("http://drive.google.com/drive/folders/This_is_a_folder_name"));
        assert!(regexes[0]
            .is_match("https://drive.google.com/drive/folders/folder-with-dashes-and-numbers-123"));

        assert!(
            regexes[1].is_match("https://drive.google.com/drive/u/0/folders/Important_Documents")
        );
        assert!(
            regexes[1].is_match("http://docs.google.com/drive/u/1234567890/folders/Shared_with_me")
        );
        assert!(
            regexes[1].is_match("https://docs.google.com/drive/u/999/folders/Project-Files-2023")
        );

        assert!(
            regexes[2].is_match("http://drive.google.com/drive/mobile/folders/Photos_from_trip")
        );
        assert!(regexes[2].is_match("https://docs.google.com/drive/mobile/folders/Offline_Files"));
        assert!(regexes[2]
            .is_match("https://drive.google.com/drive/mobile/folders/My_Music_Collection"));
    })
    .await
}
