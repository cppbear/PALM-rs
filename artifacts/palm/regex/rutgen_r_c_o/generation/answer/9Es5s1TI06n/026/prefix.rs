// Answer 0

#[test]
fn test_class_unclosed() {
    let error = ErrorKind::ClassUnclosed;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
}

#[test]
fn test_class_unclosed_multiple() {
    let error = ErrorKind::ClassUnclosed;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
}

