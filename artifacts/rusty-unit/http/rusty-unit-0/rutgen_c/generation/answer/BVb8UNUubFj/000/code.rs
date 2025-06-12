// Answer 0

#[test]
fn test_version_set_http_1_1() {
    let builder = Builder::new();
    let req = builder.version(Version(Http::HTTP_1_1)).body(()).unwrap();
    assert_eq!(req.version, Version(Http::HTTP_1_1));
}

#[test]
fn test_version_set_http_2() {
    let builder = Builder::new();
    let req = builder.version(Version(Http::HTTP_2)).body(()).unwrap();
    assert_eq!(req.version, Version(Http::HTTP_2));
}

#[test]
fn test_version_set_http_3() {
    let builder = Builder::new();
    let req = builder.version(Version(Http::HTTP_3)).body(()).unwrap();
    assert_eq!(req.version, Version(Http::HTTP_3));
}

#[should_panic]
fn test_version_invalid_state() {
    let builder = Builder::new();
    let _ = builder.version(Version(Http::HTTP_2)).body(()).unwrap();  // Assuming body requires a valid state
}

