// Answer 0

#[test]
fn test_visit_seq_with_value_0() {
    struct MockSeq {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as u8)) // corresponds with expectation
            } else {
                Ok(None)
            }
        }
    }

    let seq = MockSeq { values: vec![0], index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_value_1() {
    struct MockSeq {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as u8))
            } else {
                Ok(None)
            }
        }
    }

    let seq = MockSeq { values: vec![1], index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_value_100() {
    struct MockSeq {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as u8))
            } else {
                Ok(None)
            }
        }
    }

    let seq = MockSeq { values: vec![100], index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_value_200() {
    struct MockSeq {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as u8))
            } else {
                Ok(None)
            }
        }
    }

    let seq = MockSeq { values: vec![200], index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_err() {
    struct MockSeq {
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err(crate::de::Error::custom("mock error"))
        }
    }

    let seq = MockSeq { index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_with_none() {
    struct MockSeq {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeq {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as u8))
            } else {
                Ok(None)
            }
        }
    }

    let seq = MockSeq { values: vec![], index: 0 };
    let visitor = RangeToVisitor { expecting: "an end value", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

