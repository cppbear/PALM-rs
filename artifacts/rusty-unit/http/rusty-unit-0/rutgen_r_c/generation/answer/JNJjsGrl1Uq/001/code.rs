// Answer 0

#[test]
fn test_version() {
    struct MockResponse {
        head: Parts,
    }

    impl MockResponse {
        fn new(version: Version) -> Self {
            MockResponse {
                head: Parts {
                    version,
                    ..Default::default()
                },
            }
        }

        fn version(&self) -> Version {
            self.head.version
        }
    }

    let response_http_10 = MockResponse::new(Version(Http(0)));
    assert_eq!(response_http_10.version(), Version(Http(0))); // HTTP/1.0

    let response_http_11 = MockResponse::new(Version(Http(1)));
    assert_eq!(response_http_11.version(), Version(Http(1))); // HTTP/1.1

    let response_http_2 = MockResponse::new(Version(Http(2)));
    assert_eq!(response_http_2.version(), Version(Http(2))); // HTTP/2.0
}

