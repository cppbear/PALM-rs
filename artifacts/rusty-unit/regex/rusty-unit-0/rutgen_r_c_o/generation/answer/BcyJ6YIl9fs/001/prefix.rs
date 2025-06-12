// Answer 0

#[test]
fn test_is_always_utf8_bytes_empty() {
    let bytes_class = Class::Bytes(ClassBytes::empty());
    bytes_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_bytes_asciirange() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0, 127)]));
    bytes_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_bytes_valid_range() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0, 100),
        ClassBytesRange::new(101, 127),
    ]));
    bytes_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_bytes_single_byte() {
    let mut bytes_class = Class::Bytes(ClassBytes::empty());
    bytes_class.push(ClassBytesRange::new(0, 63));
    bytes_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_bytes_non_ascii_range() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(128, 255)]));
    bytes_class.is_always_utf8();
}

#[test]
fn test_is_always_utf8_bytes_mixed_ranges() {
    let mut bytes_class = Class::Bytes(ClassBytes::empty());
    bytes_class.push(ClassBytesRange::new(0, 127));
    bytes_class.push(ClassBytesRange::new(128, 255));
    bytes_class.is_always_utf8();
}

