// Answer 0

#[test]
fn test_builder_new() {
    let builder = http::uri::Builder::new();
    assert_eq!(builder.build().is_err(), true); // Since no scheme, authority, or path is set, it should fail to build
}

#[test]
fn test_builder_with_scheme() {
    let builder = http::uri::Builder::new()
        .scheme("https");
    assert_eq!(builder.build().is_err(), true); // Still missing authority and path, should fail
}

#[test]
fn test_builder_with_authority() {
    let builder = http::uri::Builder::new()
        .authority("hyper.rs");
    assert_eq!(builder.build().is_err(), true); // Still missing scheme and path, should fail
}

#[test]
fn test_builder_with_path() {
    let builder = http::uri::Builder::new()
        .path_and_query("/");
    assert_eq!(builder.build().is_err(), true); // Still missing scheme and authority, should fail
}

#[test]
fn test_builder_complete_uri() {
    let uri = http::uri::Builder::new()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/")
        .build()
        .unwrap();
    assert_eq!(uri.to_string(), "https://hyper.rs/"); // Expect the correct URI string
}

