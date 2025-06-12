// Answer 0

#[test]
fn test_debug_version_non_exhaustive() {
    struct TestVersion(Version);
    let version = TestVersion(Version(Http::__NonExhaustive));
    let _ = format!("{:?}", version);
}

#[test]
#[should_panic]
fn test_debug_version_panic() {
    struct TestVersion(Version);
    let version = TestVersion(Version(Http::__NonExhaustive));
    let _ = format!("{:?}", version);
}

