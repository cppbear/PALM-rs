// Answer 0

#[test]
fn test_new_invalid_header_name() {
    struct InvalidHeaderName {
        _priv: (),
    }
    
    impl InvalidHeaderName {
        pub(super) fn new() -> InvalidHeaderName {
            InvalidHeaderName { _priv: () }
        }
    }

    let header_name = InvalidHeaderName::new();
    assert_eq!(header_name._priv, ());
}

#[test]
fn test_new_invalid_header_name_empty() {
    struct InvalidHeaderName {
        _priv: (),
    }
    
    impl InvalidHeaderName {
        pub(super) fn new() -> InvalidHeaderName {
            InvalidHeaderName { _priv: () }
        }
    }

    // Testing the same function without any value change to see it returns valid object
    let header_name = InvalidHeaderName::new();
    assert_eq!(header_name._priv, ());
}

