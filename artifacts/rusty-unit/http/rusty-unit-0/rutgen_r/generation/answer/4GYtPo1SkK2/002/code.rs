// Answer 0

#[derive(Debug, PartialEq)]
pub enum Scheme2 {
    None,
    Http,
    Https,
}

struct TestStruct {
    scheme: Scheme2,
}

impl TestStruct {
    pub fn is_none(&self) -> bool {
        matches!(self.scheme, Scheme2::None)
    }
}

#[test]
fn test_is_none_returns_true() {
    let test_instance = TestStruct {
        scheme: Scheme2::None,
    };
    assert!(test_instance.is_none());
}

#[test]
fn test_is_none_returns_false_for_http() {
    let test_instance = TestStruct {
        scheme: Scheme2::Http,
    };
    assert!(!test_instance.is_none());
}

#[test]
fn test_is_none_returns_false_for_https() {
    let test_instance = TestStruct {
        scheme: Scheme2::Https,
    };
    assert!(!test_instance.is_none());
}

