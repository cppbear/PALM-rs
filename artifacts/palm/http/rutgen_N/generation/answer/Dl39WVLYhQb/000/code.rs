// Answer 0

#[derive(Debug)]
struct InvalidHeaderName {
    _priv: (),
}

#[test]
fn test_new_invalid_header_name() {
    let header_name = new();
    assert_eq!(format!("{:?}", header_name), "InvalidHeaderName { _priv: () }");
}

