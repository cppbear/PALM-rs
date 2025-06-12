// Answer 0

#[test]
fn test_visit_seq_with_valid_sequence() {
    struct MockSeqAccess {
        elements: Vec<Option<Content>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = &'static str;

        fn next_element_seed<T>(
            &mut self,
            _: T,
        ) -> Result<Option<Content>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index >= self.elements.len() {
                return Ok(None);
            }
            let elem = self.elements[self.index].take();
            self.index += 1;
            Ok(elem)
        }
    }

    struct MockVisitor {
        tag_name: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = (Content, Vec<Content>);

        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let tag = match seq.next_element_seed(MockTagVisitor {})? {
                Some(tag) => tag,
                None => return Err("missing field"),
            };
            let mut rest = Vec::new();
            while let Some(elem) = seq.next_element_seed(MockContentVisitor {})? {
                rest.push(elem);
            }
            Ok((tag, rest))
        }
    }

    struct MockTagVisitor;

    impl<'de> DeserializeSeed<'de> for MockTagVisitor {
        type Value = Content;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Content::String("tag".to_owned()))
        }
    }

    struct MockContentVisitor;

    impl<'de> DeserializeSeed<'de> for MockContentVisitor {
        type Value = Content;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Content::U8(1))
        }
    }

    let sequence = MockSeqAccess {
        elements: vec![Some(Content::String("tag".to_owned())), Some(Content::U8(1))],
        index: 0,
    };

    let visitor = MockVisitor {
        tag_name: "tag",
    };

    let result = visitor.visit_seq(sequence);
    assert!(result.is_ok());
    let (tag, content) = result.unwrap();
    assert_eq!(tag, Content::String("tag".to_owned()));
    assert_eq!(content.len(), 1);
    assert_eq!(content[0], Content::U8(1));
}

#[test]
fn test_visit_seq_with_missing_tag() {
    struct MockSeqAccess {
        elements: Vec<Option<Content>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = &'static str;

        fn next_element_seed<T>(
            &mut self,
            _: T,
        ) -> Result<Option<Content>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index >= self.elements.len() {
                return Ok(None);
            }
            let elem = self.elements[self.index].take();
            self.index += 1;
            Ok(elem)
        }
    }

    struct MockVisitor {
        tag_name: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = (Content, Vec<Content>);

        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let tag = match seq.next_element_seed(MockTagVisitor {})? {
                Some(tag) => tag,
                None => return Err("missing field"),
            };
            let mut rest = Vec::new();
            while let Some(elem) = seq.next_element_seed(MockContentVisitor {})? {
                rest.push(elem);
            }
            Ok((tag, rest))
        }
    }

    struct MockTagVisitor;

    impl<'de> DeserializeSeed<'de> for MockTagVisitor {
        type Value = Content;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err("tag deserialization error")
        }
    }

    let sequence = MockSeqAccess {
        elements: vec![None],
        index: 0,
    };

    let visitor = MockVisitor {
        tag_name: "tag",
    };

    let result = visitor.visit_seq(sequence);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "missing field");
}

