// Answer 0

#[test]
fn test_unexpected_with_newtype_struct() {
    struct TestNewtype(Content<'static>);

    let content = TestNewtype(Content::NewtypeStruct("TestStruct", Box::new(Content::Bool(true))));

    let result = content.0.unexpected();
    assert_eq!(result, Unexpected::NewtypeStruct);
}

