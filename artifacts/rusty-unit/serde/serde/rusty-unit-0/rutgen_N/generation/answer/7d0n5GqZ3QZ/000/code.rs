// Answer 0

#[derive(Debug)]
struct TestSeqAccess {
    elements: Vec<i32>,
    index: usize,
}

impl TestSeqAccess {
    fn new(elements: Vec<i32>) -> Self {
        TestSeqAccess { elements, index: 0 }
    }
}

impl<'de> SeqAccess<'de> for TestSeqAccess {
    type Error = serde::de::value::Error;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        if self.index < self.elements.len() {
            let value: T = Deserialize::deserialize(&mut self.elements[self.index]).map_err(|_| Self::Error::custom("Deserialization failed"))?;
            self.index += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_seq_with_valid_element() {
    let seq_access = TestSeqAccess::new(vec![42]);
    let result: Result<i32, _> = visit_seq(seq_access);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_seq_with_no_elements() {
    let seq_access = TestSeqAccess::new(vec![]);
    let result: Result<i32, _> = visit_seq(seq_access);
    assert!(result.is_err());
}

