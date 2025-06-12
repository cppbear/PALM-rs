// Answer 0

#[test]
fn test_visit_seq_with_valid_bool_tag() {
    struct TestSeq {
        values: Vec<Content>,
        index: usize,
    }

    impl SeqAccess<'_> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            if self.index < self.values.len() {
                self.index += 1;
                Ok(Some(self.values[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        values: vec![Content::Bool(true), Content::String("value".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a bool tag",
        value: PhantomData,
    };

    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_valid_u8_tag() {
    struct TestSeq {
        values: Vec<Content>,
        index: usize,
    }

    impl SeqAccess<'_> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            if self.index < self.values.len() {
                self.index += 1;
                Ok(Some(self.values[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        values: vec![Content::U8(255), Content::Bytes(vec![1, 2, 3])],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "an u8 tag",
        value: PhantomData,
    };

    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_missing_tag() {
    struct TestSeq {
        values: Vec<Content>,
        index: usize,
    }

    impl SeqAccess<'_> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            Ok(None)
        }
    }

    let seq = TestSeq {
        values: vec![],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a missing tag",
        value: PhantomData,
    };

    let result = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_valid_string_tag() {
    struct TestSeq {
        values: Vec<Content>,
        index: usize,
    }

    impl SeqAccess<'_> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            if self.index < self.values.len() {
                self.index += 1;
                Ok(Some(self.values[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        values: vec![Content::String("tag_value".to_string()), Content::F64(10.5)],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a string tag",
        value: PhantomData,
    };

    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_invalid_tag() {
    struct TestSeq {
        values: Vec<Content>,
        index: usize,
    }

    impl SeqAccess<'_> for TestSeq {
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            if self.index < self.values.len() {
                self.index += 1;
                Ok(Some(self.values[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        values: vec![Content::None, Content::String("value".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "an invalid tag scenario",
        value: PhantomData,
    };

    let result = visitor.visit_seq(seq);
}

