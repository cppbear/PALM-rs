// Answer 0

#[test]
fn test_status_default_response() {
    use std::num::NonZeroU16;

    struct MockBody;

    impl Default for Response<MockBody> {
        fn default() -> Self {
            let default_parts = Parts {
                status: StatusCode(NonZeroU16::new(200).unwrap()), // OK status
                version: Version::default(),
                headers: HeaderMap::new(),
                extensions: Extensions::default(),
                _priv: (),
            };
            Response {
                head: default_parts,
                body: MockBody,
            }
        }
    }

    let response: Response<MockBody> = Response::default();
    assert_eq!(response.status(), StatusCode(NonZeroU16::new(200).unwrap()));
}

#[test]
fn test_status_custom_response() {
    use std::num::NonZeroU16;

    struct CustomBody;

    let custom_parts = Parts {
        status: StatusCode(NonZeroU16::new(404).unwrap()), // Not Found status
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let response: Response<CustomBody> = Response::from_parts(custom_parts, CustomBody);

    assert_eq!(response.status(), StatusCode(NonZeroU16::new(404).unwrap()));
}

