// Answer 0

#[test]
fn test_visit_seq_success() {
    struct MockSeqAccess {
        elements: Vec<Option<Content>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = ();
        
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value = self.elements[self.index].take();
                self.index += 1;
                Ok(value)
            } else {
                Ok(None)
            }
        }
    }

    impl Deserialize<'_> for Content {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'_>,
        {
            // Simulating successful deserialization for testing
            // Replace this with real deserialization logic if needed
            Ok(Content::Bool(true))
        }
    }

    let seq = MockSeqAccess {
        elements: vec![Some(Content::Bool(true))],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "expecting a tag",
        value: PhantomData,
    };

    let result = visitor.visit_seq(seq).unwrap();
    assert!(matches!(result.0, Content::Bool(true)));
}

#[test]
fn test_visit_seq_missing_field() {
    struct MockSeqAccess {
        elements: Vec<Option<Content>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let seq = MockSeqAccess {
        elements: vec![],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "expecting a tag",
        value: PhantomData,
    };

    let result = visitor.visit_seq(seq);
    assert!(result.is_err());
}

#[test]
fn test_visit_seq_deserialize_error() {
    struct MockSeqAccess {
        elements: Vec<Option<Content>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value = self.elements[self.index].take();
                self.index += 1;
                Ok(value)
            } else {
                Ok(None)
            }
        }
    }

    impl Deserialize<'_> for Content {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'_>,
        {
            Err(())
        }
    }

    let seq = MockSeqAccess {
        elements: vec![Some(Content::Bool(true))],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "expecting a tag",
        value: PhantomData,
    };

    let result = visitor.visit_seq(seq);
    assert!(result.is_err());
}

