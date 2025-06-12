// Answer 0

#[test]
fn test_class_range_invalid() {
    let error = ErrorKind::ClassRangeInvalid;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
}

#[test]
fn test_class_range_invalid_with_zero_boundary() {
    let error = ErrorKind::ClassRangeInvalid;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
}

