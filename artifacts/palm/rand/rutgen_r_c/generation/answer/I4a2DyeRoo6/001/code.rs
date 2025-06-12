// Answer 0

#[test]
fn test_empty_display() {
    let empty = Empty;
    let mut output = core::fmt::Formatter::new();
    let result = empty.fmt(&mut output);
    assert!(result.is_ok());
}

#[test]
fn test_empty_display_output() {
    let empty = Empty;
    let mut output = String::new();
    {
        let mut formatter = core::fmt::Formatter::new();
        let _ = empty.fmt(&mut formatter);
        output = formatter.to_string();
    }
    assert_eq!(output, "Tried to create a `rand::distr::slice::Choose` with an empty slice");
}

