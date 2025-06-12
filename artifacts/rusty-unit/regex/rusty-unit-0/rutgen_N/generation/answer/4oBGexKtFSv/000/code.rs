// Answer 0

#[test]
fn test_new_valid_regex() {
    let result = new(r"\d+");
    assert!(result.is_ok());
    let regex = result.unwrap();
    assert!(regex.is_match("123"));
}

#[test]
#[should_panic]
fn test_new_invalid_regex() {
    let result = new(r"([");
    assert!(result.is_err());
}

