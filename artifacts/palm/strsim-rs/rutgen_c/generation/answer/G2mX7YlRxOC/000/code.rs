// Answer 0

#[test]
fn test_strsim_error_display() {
    let error = StrSimError::DifferentLengthArgs;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Differing length arguments provided");
}

