// Answer 0

#[test]
fn test_fmt_success() {
    let error = TryGetError { requested: 10, available: 5 };
    let mut output = Vec::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(
        String::from_utf8_lossy(&output),
        "Not enough bytes remaining in buffer to read value (requested 10 but only 5 available)"
    );
}

#[test]
fn test_fmt_boundary_values() {
    let error_zero = TryGetError { requested: 0, available: 0 };
    let mut output_zero = Vec::new();
    let result_zero = write!(&mut output_zero, "{}", error_zero);
    assert!(result_zero.is_ok());
    assert_eq!(
        String::from_utf8_lossy(&output_zero),
        "Not enough bytes remaining in buffer to read value (requested 0 but only 0 available)"
    );

    let error_large = TryGetError { requested: usize::MAX, available: usize::MAX };
    let mut output_large = Vec::new();
    let result_large = write!(&mut output_large, "{}", error_large);
    assert!(result_large.is_ok());
    assert_eq!(
        String::from_utf8_lossy(&output_large),
        format!("Not enough bytes remaining in buffer to read value (requested {} but only {} available)", usize::MAX, usize::MAX)
    );
}

