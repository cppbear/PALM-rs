// Answer 0

#[test]
fn test_visit_seq_valid_case() {
    struct ValidSeqAccess;
    
    impl<'de> SeqAccess<'de> for ValidSeqAccess {
        type Error = &'static str;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(Some("valid_tag"))
        }
    }

    let visitor = TaggedContentVisitor { tag_name: "tag", expecting: "a valid tag", value: PhantomData::<()>::default() };
    let mut seq = ValidSeqAccess;

    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_no_element() {
    struct NoElementSeqAccess;

    impl<'de> SeqAccess<'de> for NoElementSeqAccess {
        type Error = &'static str;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let visitor = TaggedContentVisitor { tag_name: "tag", expecting: "a valid tag", value: PhantomData::<()>::default() };
    let mut seq = NoElementSeqAccess;

    let result = visitor.visit_seq(seq);
    let _ = result.map_err(|err| {
        assert_eq!(err, "missing field 'tag'");
    });
}

#[test]
fn test_visit_seq_error_on_next_element() {
    struct ErrorSeqAccess;

    impl<'de> SeqAccess<'de> for ErrorSeqAccess {
        type Error = &'static str;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err("next_element error")
        }
    }

    let visitor = TaggedContentVisitor { tag_name: "tag", expecting: "a valid tag", value: PhantomData::<()>::default() };
    let mut seq = ErrorSeqAccess;

    let result = visitor.visit_seq(seq);
    let _ = result.map_err(|err| {
        assert_eq!(err, "next_element error");
    });
}

#[test]
fn test_visit_seq_deserialize_error() {
    struct ValidSeqAccess;
    
    impl<'de> SeqAccess<'de> for ValidSeqAccess {
        type Error = &'static str;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(Some("valid_tag"))
        }
    }

    struct ErrorContentDeserializer;

    impl<'de> Deserialize<'de> for Content<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err("Content deserialize error")
        }
    }

    let visitor = TaggedContentVisitor { tag_name: "tag", expecting: "a valid tag", value: PhantomData::<()>::default() };
    let mut seq = ValidSeqAccess;

    let result = visitor.visit_seq(seq);
    let _ = result.map_err(|err| {
        assert_eq!(err, "Content deserialize error");
    });
}

