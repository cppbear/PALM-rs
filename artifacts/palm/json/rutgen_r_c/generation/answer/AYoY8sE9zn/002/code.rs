// Answer 0

#[test]
fn test_size_hint_with_equal_lower_upper() {
    struct DummyVisitor;
    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = usize;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number")
        }
    }

    struct TestMapDeserializer {
        iter: std::iter::Once<(String, Value)>,
    }

    impl TestMapDeserializer {
        fn new() -> Self {
            let iter = std::iter::once((String::from("key"), Value::Number(Number::from(42))));
            TestMapDeserializer { iter }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let deserializer = TestMapDeserializer::new();
    assert_eq!(deserializer.size_hint(), Some(1));
}

#[test]
fn test_size_hint_with_no_elements() {
    struct TestMapDeserializer {
        iter: std::iter::Empty<(String, Value)>,
    }

    impl TestMapDeserializer {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let deserializer = TestMapDeserializer { iter: std::iter::empty() };
    assert_eq!(deserializer.size_hint(), None);
}

