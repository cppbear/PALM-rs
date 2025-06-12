// Answer 0

#[test]
fn test_version_http10() {
    let version = Version(Http::Http10);
    let mut output = Vec::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http11() {
    let version = Version(Http::Http11);
    let mut output = Vec::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_http09() {
    let version = Version(Http::Http09);
    let mut output = Vec::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_h2() {
    let version = Version(Http::H2);
    let mut output = Vec::new();
    let result = version.fmt(&mut output);
}

#[test]
fn test_version_h3() {
    let version = Version(Http::H3);
    let mut output = Vec::new();
    let result = version.fmt(&mut output);
}

