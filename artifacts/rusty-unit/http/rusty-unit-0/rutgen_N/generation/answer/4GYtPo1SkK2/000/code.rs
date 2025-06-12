// Answer 0

#[derive(Debug)]
enum Scheme2 {
    None,
    Http,
    Https,
}

impl Scheme2 {
    pub fn is_none(&self) -> bool {
        matches!(*self, Scheme2::None)
    }
}

#[test]
fn test_is_none_when_none() {
    let scheme = Scheme2::None;
    assert!(scheme.is_none());
}

#[test]
fn test_is_none_when_http() {
    let scheme = Scheme2::Http;
    assert!(!scheme.is_none());
}

#[test]
fn test_is_none_when_https() {
    let scheme = Scheme2::Https;
    assert!(!scheme.is_none());
}

