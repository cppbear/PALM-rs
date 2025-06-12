// Answer 0

#[test]
fn test_version_http_2() {
    let builder = Builder::new();
    let version = Version(Http::HTTP_2);
    let _ = builder.version(version);
}

#[test]
fn test_version_http_1_1() {
    let builder = Builder::new();
    let version = Version(Http::HTTP_1_1);
    let _ = builder.version(version);
}

#[test]
fn test_version_http_3() {
    let builder = Builder::new();
    let version = Version(Http::HTTP_3);
    let _ = builder.version(version);
}

#[test]
fn test_version_http_0_9() {
    let builder = Builder::new();
    let version = Version(Http::HTTP_0_9);
    let _ = builder.version(version);
}

#[test]
fn test_version_http_1_0() {
    let builder = Builder::new();
    let version = Version(Http::HTTP_1_0);
    let _ = builder.version(version);
}

