// Answer 0

#[test]
fn test_version() {
    struct TestBody;
    impl Default for Response<TestBody> {
        fn default() -> Self {
            Response {
                head: Parts {
                    status: StatusCode::OK,
                    version: Version::HTTP_11,
                    headers: HeaderMap::new(),
                    extensions: Extensions::new(),
                    _priv: (),
                },
                body: TestBody,
            }
        }
    }
    
    let response: Response<TestBody> = Response::default();
    assert_eq!(response.version(), Version::HTTP_11);
}

