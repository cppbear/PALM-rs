// Answer 0

#[test]
fn test_borrow_standard_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let standard_header = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept),
    };
    
    assert_eq!(standard_header.borrow(), "accept");
}

#[test]
fn test_borrow_custom_header() {
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct CustomHeader(ByteStr);

    let custom_header = HeaderName {
        inner: Repr::Custom(CustomHeader(ByteStr::from("custom-header"))),
    };
    
    assert_eq!(custom_header.borrow(), "custom-header");
}

#[test]
#[should_panic]
fn test_borrow_invalid_standard_header() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct InvalidHeader; // Dummy struct to trigger panic

    let invalid_header = HeaderName {
        inner: Repr::Custom(InvalidHeader),
    };

    let _ = invalid_header.borrow(); // This should panic as it's not a valid HeaderName
}

#[test]
fn test_borrow_empty_custom_header() {
    let empty_custom_header = HeaderName {
        inner: Repr::Custom(Custom(ByteStr::from(""))),
    };
    
    assert_eq!(empty_custom_header.borrow(), "");
}

