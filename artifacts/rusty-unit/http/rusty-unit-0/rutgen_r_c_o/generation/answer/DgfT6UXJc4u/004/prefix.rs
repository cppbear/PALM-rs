// Answer 0

#[test]
fn test_version_http11_debug() {
    let version = Version(Http::Http11);
    let mut output = String::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http10_debug() {
    let version = Version(Http::Http10);
    let mut output = String::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http09_debug() {
    let version = Version(Http::Http09);
    let mut output = String::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http2_debug() {
    let version = Version(Http::H2);
    let mut output = String::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http3_debug() {
    let version = Version(Http::H3);
    let mut output = String::new();
    let result = version.fmt(&mut output);
}

