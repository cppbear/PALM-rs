// Answer 0

#[test]
fn test_no_expansion_without_dollar() {
    let mut input: &str = "static string without dollar sign";
    let result = input.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(input)));
}

#[test]
fn test_no_expansion_with_dollar() {
    let mut input: &str = "string with $ dollar sign";
    let result = input.no_expansion();
    assert_eq!(result, None);
}

#[test]
fn test_no_expansion_empty_string() {
    let mut input: &str = "";
    let result = input.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(input)));
}

#[test]
fn test_no_expansion_single_dollar() {
    let mut input: &str = "$";
    let result = input.no_expansion();
    assert_eq!(result, None);
}

#[test]
fn test_no_expansion_multiple_dollars() {
    let mut input: &str = "this is a $ test$ string";
    let result = input.no_expansion();
    assert_eq!(result, None);
}

