// Answer 0

#[test]
fn test_try_get_error_display() {
    let error = TryGetError { requested: 10, available: 5 };
    let mut output = Vec::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        assert!(error.fmt(&mut formatter).is_ok());
    }
    let result = String::from_utf8(output).unwrap();
    assert_eq!(
        result,
        "Not enough bytes remaining in buffer to read value (requested 10 but only 5 available)"
    );
}

#[test]
fn test_try_get_error_display_zero_available() {
    let error = TryGetError { requested: 8, available: 0 };
    let mut output = Vec::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        assert!(error.fmt(&mut formatter).is_ok());
    }
    let result = String::from_utf8(output).unwrap();
    assert_eq!(
        result,
        "Not enough bytes remaining in buffer to read value (requested 8 but only 0 available)"
    );
}

#[test]
fn test_try_get_error_display_equal_requested_available() {
    let error = TryGetError { requested: 5, available: 5 };
    let mut output = Vec::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        assert!(error.fmt(&mut formatter).is_ok());
    }
    let result = String::from_utf8(output).unwrap();
    assert_eq!(
        result,
        "Not enough bytes remaining in buffer to read value (requested 5 but only 5 available)"
    );
} 

#[test]
fn test_try_get_error_display_large_values() {
    let error = TryGetError { requested: 1000, available: 999 };
    let mut output = Vec::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        assert!(error.fmt(&mut formatter).is_ok());
    }
    let result = String::from_utf8(output).unwrap();
    assert_eq!(
        result,
        "Not enough bytes remaining in buffer to read value (requested 1000 but only 999 available)"
    );
}

