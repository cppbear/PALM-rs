// Answer 0

#[test]
fn test_scheme_try_from_http() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::try_from(b"http");
}

#[test]
fn test_scheme_try_from_https() {
    let scheme: Result<Scheme, InvalidUri> = Scheme::try_from(b"https");
}

