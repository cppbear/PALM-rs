// Answer 0

#[test]
fn test_class_bytes_range_start_min() {
    let range = ClassBytesRange::new(0, 255);
    range.start();
}

#[test]
fn test_class_bytes_range_start_mid() {
    let range = ClassBytesRange::new(128, 255);
    range.start();
}

#[test]
fn test_class_bytes_range_start_max() {
    let range = ClassBytesRange::new(255, 255);
    range.start();
}

#[test]
fn test_class_bytes_range_start_alternate() {
    let range = ClassBytesRange::new(100, 200);
    range.start();
}

#[test]
fn test_class_bytes_range_start_single() {
    let range = ClassBytesRange::new(50, 50);
    range.start();
}

#[test]
fn test_class_bytes_range_start_zero() {
    let range = ClassBytesRange::new(0, 0);
    range.start();
}

