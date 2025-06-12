// Answer 0

#[test]
fn test_no_expansion_borrowed() {
    use std::borrow::Cow;

    struct TestStruct(&'static str);

    let mut test_instance = TestStruct("test string");
    let result = test_instance.no_expansion();
    
    assert_eq!(result, Some(Cow::Borrowed("test string")));
}

#[test]
fn test_no_expansion_empty() {
    use std::borrow::Cow;

    struct TestStruct(&'static str);

    let mut test_instance = TestStruct("");
    let result = test_instance.no_expansion();
    
    assert_eq!(result, Some(Cow::Borrowed("")));
}

