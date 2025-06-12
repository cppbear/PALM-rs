// Answer 0

#[test]
fn test_uri_builder_success() {
    let uri = http::Uri::builder()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/")
        .build()
        .unwrap();
    
    assert_eq!(uri.to_string(), "https://hyper.rs/");
}

#[test]
#[should_panic(expected = "invalid scheme")]
fn test_uri_builder_invalid_scheme() {
    let _uri = http::Uri::builder()
        .scheme("invalid_scheme")
        .authority("hyper.rs")
        .path_and_query("/")
        .build()
        .unwrap();
}

#[test]
fn test_uri_builder_with_empty_path() {
    let uri = http::Uri::builder()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("")
        .build()
        .unwrap();

    assert_eq!(uri.to_string(), "https://hyper.rs/");
}

#[test]
#[should_panic(expected = "invalid authority")]
fn test_uri_builder_invalid_authority() {
    let _uri = http::Uri::builder()
        .scheme("https")
        .authority("")
        .path_and_query("/")
        .build()
        .unwrap();
}

#[test]
fn test_uri_builder_with_query() {
    let uri = http::Uri::builder()
        .scheme("https")
        .authority("hyper.rs")
        .path_and_query("/?key=value")
        .build()
        .unwrap();

    assert_eq!(uri.to_string(), "https://hyper.rs/?key=value");
}

