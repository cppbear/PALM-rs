// Answer 0

#[test]
fn test_fmt_http3() {
    let version = Version(Http::H3);
    let mut buffer = String::new();
    let result = version.fmt(&mut buffer);
}

#[test]
fn test_fmt_http2() {
    let version = Version(Http::H2);
    let mut buffer = String::new();
    let result = version.fmt(&mut buffer);
}

#[test]
fn test_fmt_http11() {
    let version = Version(Http::Http11);
    let mut buffer = String::new();
    let result = version.fmt(&mut buffer);
}

#[test]
fn test_fmt_http10() {
    let version = Version(Http::Http10);
    let mut buffer = String::new();
    let result = version.fmt(&mut buffer);
}

#[test]
fn test_fmt_http09() {
    let version = Version(Http::Http09);
    let mut buffer = String::new();
    let result = version.fmt(&mut buffer);
}

