// Answer 0

#[derive(Debug)]
struct TestSeqAccess {
    elements: Vec<Idx>,
    current: usize,
}

impl<'de> SeqAccess<'de> for TestSeqAccess {
    type Error = serde::de::value::Error;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        if self.current < self.elements.len() {
            let value = self.elements[self.current].clone();
            self.current += 1;
            Ok(Some(value as T))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_seq_valid() {
    let mut seq = TestSeqAccess {
        elements: vec![1, 2, 3],
        current: 0,
    };
    
    let result: Result<Idx, _> = visit_seq(seq);
    assert_eq!(result, Ok(1));
}

#[test]
#[should_panic(expected = "invalid length")]
fn test_visit_seq_empty() {
    let mut seq = TestSeqAccess {
        elements: vec![],
        current: 0,
    };

    let _result: Result<Idx, _> = visit_seq(seq);
}

