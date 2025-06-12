// Answer 0

#[test]
fn test_visit_seq_empty() {
    struct SeqAccessEmpty;

    impl<'de> de::SeqAccess<'de> for SeqAccessEmpty {
        type Error = ();
        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }

    let visitor = TagOrContentVisitor {
        name: "tag",
        value: PhantomData,
    };

    let result = visitor.visit_seq(SeqAccessEmpty);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), TagOrContent::Content(Content::Seq(vec![]))));
}

#[test]
fn test_visit_seq_with_elements() {
    struct SeqAccessWithElements {
        current: usize,
        max: usize,
    }

    impl<'de> de::SeqAccess<'de> for SeqAccessWithElements {
        type Error = ();
        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.current < self.max {
                self.current += 1;
                let value = match self.current {
                    1 => Content::Bool(true),
                    2 => Content::I32(42),
                    3 => Content::String("hello".to_string()),
                    _ => unreachable!(),
                };
                Ok(Some(seed.deserialize(value)))
            } else {
                Ok(None)
            }
        }
    }

    let visitor = TagOrContentVisitor {
        name: "tag",
        value: PhantomData,
    };

    let mut seq_access = SeqAccessWithElements { current: 0, max: 3 };
    let result = visitor.visit_seq(seq_access);
    assert!(result.is_ok());
    if let TagOrContent::Content(Content::Seq(elements)) = result.unwrap() {
        assert_eq!(elements.len(), 3);
        assert!(matches!(elements[0], Content::Bool(true)));
        assert!(matches!(elements[1], Content::I32(42)));
        assert!(matches!(elements[2], Content::String(ref s) if s == "hello"));
    } else {
        panic!("Expected Content::Seq");
    }
}

#[test]
#[should_panic]
fn test_visit_seq_invalid_type() {
    struct SeqAccessInvalidType;

    impl<'de> de::SeqAccess<'de> for SeqAccessInvalidType {
        type Error = ();
        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if true { // Simulating an invalid scenario
                let value = Content::Str("invalid");
                seed.deserialize(value).unwrap(); // This will cause a panic
            }
            Ok(None)
        }
    }

    let visitor = TagOrContentVisitor {
        name: "tag",
        value: PhantomData,
    };

    let seq_access = SeqAccessInvalidType;
    visitor.visit_seq(seq_access).unwrap();
}

