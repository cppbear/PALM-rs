// Answer 0

#[test]
fn test_end_with_empty_elements() {
    struct TestError;
    impl std::fmt::Debug for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for TestError {}

    let serialize_seq = SerializeSeq::<TestError> {
        elements: Vec::new(),
        error: std::marker::PhantomData,
    };
    let result = serialize_seq.end();
    assert_eq!(result, Ok(Content::Seq(vec![])));
}

#[test]
fn test_end_with_bool_element() {
    struct TestError;
    impl std::fmt::Debug for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for TestError {}

    let mut serialize_seq = SerializeSeq::<TestError> {
        elements: Vec::new(),
        error: std::marker::PhantomData,
    };
    serialize_seq.elements.push(Content::Bool(true));
    let result = serialize_seq.end();
    assert_eq!(result, Ok(Content::Seq(vec![Content::Bool(true)])));
}

#[test]
fn test_end_with_multiple_elements() {
    struct TestError;
    impl std::fmt::Debug for TestError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for TestError {}

    let mut serialize_seq = SerializeSeq::<TestError> {
        elements: Vec::new(),
        error: std::marker::PhantomData,
    };
    serialize_seq.elements.push(Content::U8(42));
    serialize_seq.elements.push(Content::String("Hello".to_string()));
    
    let result = serialize_seq.end();
    assert_eq!(result, Ok(Content::Seq(vec![
        Content::U8(42),
        Content::String("Hello".to_string()),
    ])));
}

