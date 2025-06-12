// Answer 0

#[test]
fn test_end_empty_sequence() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for DummyError {}

    let seq: SerializeSeq<DummyError> = SerializeSeq {
        elements: Vec::new(),
        error: PhantomData,
    };
    let result = seq.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Seq(elements) = content {
            assert!(elements.is_empty());
        } else {
            panic!("Expected Content::Seq");
        }
    }
}

#[test]
fn test_end_single_element_sequence() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for DummyError {}

    let mut seq: SerializeSeq<DummyError> = SerializeSeq {
        elements: Vec::new(),
        error: PhantomData,
    };
    seq.elements.push(Content::U8(42));
    let result = seq.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Seq(elements) = content {
            assert_eq!(elements.len(), 1);
            assert_eq!(elements[0], Content::U8(42));
        } else {
            panic!("Expected Content::Seq");
        }
    }
}

#[test]
fn test_end_multiple_element_sequence() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for DummyError {}

    let mut seq: SerializeSeq<DummyError> = SerializeSeq {
        elements: Vec::new(),
        error: PhantomData,
    };
    seq.elements.push(Content::U8(42));
    seq.elements.push(Content::U64(100));
    let result = seq.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        if let Content::Seq(elements) = content {
            assert_eq!(elements.len(), 2);
            assert_eq!(elements[0], Content::U8(42));
            assert_eq!(elements[1], Content::U64(100));
        } else {
            panic!("Expected Content::Seq");
        }
    }
}

