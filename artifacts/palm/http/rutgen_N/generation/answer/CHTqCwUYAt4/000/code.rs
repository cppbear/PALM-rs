// Answer 0

#[derive(Debug)]
struct InvalidMethod {
    _priv: (),
}

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod { _priv: () }
    }
}

#[test]
fn test_invalid_method_new() {
    let method = InvalidMethod::new();
    assert_eq!(format!("{:?}", method), "InvalidMethod { _priv: () }");
}

