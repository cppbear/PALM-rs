// Answer 0

#[test]
fn test_scheme_with_http() {
    let builder = Builder::new();
    builder.scheme("http");
}

#[test]
fn test_scheme_with_https() {
    let builder = Builder::new();
    builder.scheme("https");
}

#[test]
fn test_scheme_with_ftp() {
    let builder = Builder::new();
    builder.scheme("ftp");
}

#[test]
fn test_scheme_with_mailto() {
    let builder = Builder::new();
    builder.scheme("mailto");
}

#[test]
fn test_scheme_with_file() {
    let builder = Builder::new();
    builder.scheme("file");
}

#[test]
fn test_scheme_with_ws() {
    let builder = Builder::new();
    builder.scheme("ws");
}

#[test]
fn test_scheme_with_wss() {
    let builder = Builder::new();
    builder.scheme("wss");
}

#[test]
fn test_scheme_with_custom_scheme() {
    let builder = Builder::new();
    builder.scheme("custom-scheme");
}

#[test]
#[should_panic]
fn test_scheme_with_empty_string() {
    let builder = Builder::new();
    builder.scheme("");
}

#[test]
#[should_panic]
fn test_scheme_with_invalid_scheme() {
    let builder = Builder::new();
    builder.scheme("invalid_scheme");
}

#[test]
#[should_panic]
fn test_scheme_with_integer() {
    let builder = Builder::new();
    builder.scheme(123);
}

#[test]
#[should_panic]
fn test_scheme_with_boolean() {
    let builder = Builder::new();
    builder.scheme(true);
}

