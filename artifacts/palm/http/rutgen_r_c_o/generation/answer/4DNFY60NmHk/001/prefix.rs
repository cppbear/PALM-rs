// Answer 0

#[test]
fn test_builder_default() {
    let builder = Response::builder();
}

#[test]
fn test_builder_status_200() {
    let builder = Response::builder().status(200);
}

#[test]
fn test_builder_status_404() {
    let builder = Response::builder().status(404);
}

#[test]
fn test_builder_status_500() {
    let builder = Response::builder().status(500);
}

#[test]
fn test_builder_with_header() {
    let builder = Response::builder()
        .header("X-Custom-Foo", "Bar");
}

#[test]
fn test_builder_with_long_header_key() {
    let long_key = "X-".to_string() + &"C".repeat(48); // 50 characters
    let builder = Response::builder()
        .header(long_key, "Value");
}

#[test]
fn test_builder_with_long_header_value() {
    let long_value = "Value".to_string() + &"V".repeat(995); // 1000 characters
    let builder = Response::builder()
        .header("X-Custom-Foo", long_value);
}

#[test]
fn test_builder_with_http_version_1_0() {
    let version = Version::HTTP_10;
    let builder = Response::builder().version(version);
}

#[test]
fn test_builder_with_http_version_1_1() {
    let version = Version::HTTP_11;
    let builder = Response::builder().version(version);
}

#[test]
fn test_builder_with_http_version_2_0() {
    let version = Version::HTTP_2;
    let builder = Response::builder().version(version);
}

#[test]
fn test_builder_with_body_empty() {
    let builder = Response::builder().body(()).unwrap();
}

#[test]
fn test_builder_with_body_string() {
    let builder = Response::builder().body("This is the body.").unwrap();
}

