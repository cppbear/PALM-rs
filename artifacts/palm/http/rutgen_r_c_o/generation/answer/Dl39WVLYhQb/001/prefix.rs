// Answer 0

#[test]
fn test_new_invalid_header_name() {
    let header_name = InvalidHeaderName::new();
}

#[test]
fn test_new_invalid_header_name_multiple_times() {
    for _ in 0..10 {
        let _header_name = InvalidHeaderName::new();
    }
}

#[test]
fn test_new_invalid_header_name_in_loop() {
    for _ in 0..255 {
        let _header_name = InvalidHeaderName::new();
    }
}

