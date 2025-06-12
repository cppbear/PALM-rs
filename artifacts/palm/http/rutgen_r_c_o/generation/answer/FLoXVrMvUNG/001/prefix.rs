// Answer 0

#[test]
fn test_patch_with_valid_uri_https() {
    let _ = Request::patch("https://example.com");
}

#[test]
fn test_patch_with_valid_uri_http() {
    let _ = Request::patch("http://example.org");
}

#[test]
fn test_patch_with_valid_uri_ftp() {
    let _ = Request::patch("ftp://files.com");
}

#[test]
fn test_patch_with_invalid_uri_empty() {
    let result = std::panic::catch_unwind(|| {
        let _ = Request::patch("");
    });
    assert!(result.is_err());
}

#[test]
fn test_patch_with_invalid_uri_non_ascii() {
    let _ = Request::patch("https://example.com/ümlaut");
}

#[test]
fn test_patch_with_invalid_uri_numeric_string() {
    let result = std::panic::catch_unwind(|| {
        let _ = Request::patch("123456");
    });
    assert!(result.is_err());
}

#[test]
fn test_patch_with_excessively_long_uri() {
    let long_uri = "https://example.com/".repeat(200);
    let result = std::panic::catch_unwind(|| {
        let _ = Request::patch(&long_uri);
    });
    assert!(result.is_err());
}

