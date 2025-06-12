// Answer 0

#[test]
fn test_header_name_as_str_standard() {
    struct StandardHeaderName {
        inner: Repr<Standard>,
    }

    let header_name = StandardHeaderName {
        inner: Repr::Standard("Test-Header".to_string()),
    };

    assert_eq!(header_name.as_str(), "Test-Header");
}

#[test]
fn test_header_name_as_str_custom() {
    struct CustomHeaderName {
        inner: Repr<Custom>,
    }

    let header_name = CustomHeaderName {
        inner: Repr::Custom("custom-header".to_string()),
    };

    assert_eq!(header_name.as_str(), "custom-header");
}

#[test]
fn test_header_name_as_str_empty() {
    struct EmptyHeaderName {
        inner: Repr<Custom>,
    }

    let header_name = EmptyHeaderName {
        inner: Repr::Custom("".to_string()),
    };

    assert_eq!(header_name.as_str(), "");
}

