// Answer 0

#[test]
fn test_scheme_empty() {
    struct MockByteStr;

    let scheme = Scheme::empty();
    match scheme.inner {
        Scheme2::None => {}
        _ => panic!("Expected Scheme2::None variant"),
    }
}

#[test]
fn test_scheme_http() {
    let scheme = Scheme::HTTP;
    match scheme.inner {
        Scheme2::Standard(Protocol::Http) => {}
        _ => panic!("Expected Scheme2::Standard with Http protocol"),
    }
}

#[test]
fn test_scheme_https() {
    let scheme = Scheme::HTTPS;
    match scheme.inner {
        Scheme2::Standard(Protocol::Https) => {}
        _ => panic!("Expected Scheme2::Standard with Https protocol"),
    }
}

