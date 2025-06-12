// Answer 0

#[test]
fn test_invalid_header_value_debug() {
    struct InvalidHeaderValue {
        _priv: (),
    }

    impl fmt::Debug for InvalidHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("InvalidHeaderValue")
                // skip _priv noise
                .finish()
        }
    }

    let value = InvalidHeaderValue { _priv: () };
    let result = format!("{:?}", value);
    assert_eq!(result, "InvalidHeaderValue");
}

