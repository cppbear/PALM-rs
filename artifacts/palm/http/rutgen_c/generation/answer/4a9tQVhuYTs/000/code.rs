// Answer 0

#[test]
fn test_version_mut_initial_value() {
    struct TestBody;
    let mut response: Response<TestBody> = Response::new(TestBody);
    assert_eq!(response.version_mut(), &mut Version(Http));
}

#[test]
fn test_version_mut_change_value() {
    struct TestBody;
    let mut response: Response<TestBody> = Response::new(TestBody);
    *response.version_mut() = Version(Http);
    assert_eq!(response.version(), Version(Http));
}

#[test]
fn test_version_mut_multiple_changes() {
    struct TestBody;
    let mut response: Response<TestBody> = Response::new(TestBody);
    *response.version_mut() = Version(Http);
    assert_eq!(response.version(), Version(Http));
    *response.version_mut() = Version(Http);
    assert_eq!(response.version(), Version(Http));
}

