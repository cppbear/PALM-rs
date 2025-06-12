// Answer 0

#[test]
fn test_no_expansion_borrowed() {
    struct Captures;
    let input_str = "Test string";
    let no_expand = NoExpand(input_str);
    let result = no_expand.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(input_str)));
}

#[test]
fn test_no_expansion_empty_string() {
    struct Captures;
    let input_str = "";
    let no_expand = NoExpand(input_str);
    let result = no_expand.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed(input_str)));
}

