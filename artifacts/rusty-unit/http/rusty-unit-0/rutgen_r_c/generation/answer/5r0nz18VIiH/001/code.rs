// Answer 0

#[test]
fn test_header_name_debug_fmt_standard() {
    let header_name = HeaderName { 
        inner: Repr::Standard(StandardHeader::Accept) 
    };
    let mut output = std::fmt::Formatter::new();
    let result = header_name.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.to_string(), "accept");
}

#[test]
fn test_header_name_debug_fmt_custom() {
    let custom_str = ByteStr::from_static(b"custom-header");
    let header_name = HeaderName { 
        inner: Repr::Custom(Custom(custom_str)) 
    };
    let mut output = std::fmt::Formatter::new();
    let result = header_name.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output.to_string(), "custom-header");
}

#[test]
#[should_panic]
fn test_header_name_debug_fmt_panic() {
    let header_name = HeaderName {
        inner: Repr::Standard(StandardHeader::Accept) 
    };
    // Creating a formatter that will inevitably panic
    let mut bad_formatter = std::fmt::Formatter::new();
    bad_formatter.write_str("will panic").unwrap(); // Simulate internal state that leads to panic
    let _ = header_name.fmt(&mut bad_formatter);
}

