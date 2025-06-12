// Answer 0

#[test]
fn test_fmt_empty() {
    let empty = Empty;
    let mut output = core::fmt::Formatter::new();
    let result = empty.fmt(&mut output);
    
    assert!(result.is_ok());
    let expected = "Tried to create a `rand::distr::slice::Choose` with an empty slice";
    assert_eq!(output.str(), expected);
}

