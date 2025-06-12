// Answer 0

#[derive(Debug)]
struct DummySeq<'de> {
    elements: Vec<Option<Idx>>,
    current: usize,
}

impl<'de> SeqAccess<'de> for DummySeq<'de> {
    type Error = serde::de::Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current < self.elements.len() {
            let result = self.elements[self.current].take();
            self.current += 1;
            match result {
                Some(value) => seed.deserialize(value),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_visit_seq_ok() {
    let seq = DummySeq {
        elements: vec![Some(Idx(1)), Some(Idx(2))],
        current: 0,
    };

    let result = visit_seq(seq);
    assert_eq!(result, Ok((Idx(1), Idx(2))));
}

#[test]
fn test_visit_seq_not_enough_elements() {
    let seq = DummySeq {
        elements: vec![Some(Idx(1))],
        current: 0,
    };

    let result = visit_seq(seq);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_visit_seq_empty() {
    let seq = DummySeq {
        elements: vec![],
        current: 0,
    };

    let result = visit_seq(seq);
    assert_eq!(result.is_err(), true);
}

