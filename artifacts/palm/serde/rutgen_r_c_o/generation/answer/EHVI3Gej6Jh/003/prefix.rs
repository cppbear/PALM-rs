// Answer 0

#[test]
fn test_visit_seq_valid_range() {
    struct TestSeq {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::value::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value: T = serde::de::from_value(serde_json::json!(self.elements[self.index])).unwrap();
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq { elements: vec![1, 10], index: 0 };
    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_duplicate_fields() {
    struct TestMap {
        keys: Vec<&'static str>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::value::Error;

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            if self.index < self.keys.len() {
                let key: K = serde::de::from_value(serde_json::json!(self.keys[self.index])).unwrap();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value: V = serde::de::from_value(serde_json::json!(self.values[self.index])).unwrap();
            self.index += 1;
            Ok(value)
        }
    }

    let map = TestMap {
        keys: vec!["start", "start", "end"],
        values: vec![1, 2, 10],
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_seq_missing_fields() {
    struct TestSeqMissing {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqMissing {
        type Error = serde::de::value::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(None)
        }
    }

    let seq = TestSeqMissing { elements: vec![], index: 0 };
    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_invalid_length_start() {
    struct TestSeqInvalid {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqInvalid {
        type Error = serde::de::value::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index == 0 {
                self.index += 1;
                let value: T = serde::de::from_value(serde_json::json!(1)).unwrap();
                return Ok(Some(value));
            }
            Ok(None)
        }
    }

    let seq = TestSeqInvalid { elements: vec![1], index: 0 };
    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_min_max_int() {
    struct TestSeqMinMax {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqMinMax {
        type Error = serde::de::value::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value: T = serde::de::from_value(serde_json::json!(self.elements[self.index])).unwrap();
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeqMinMax { elements: vec![i32::MIN, i32::MAX], index: 0 };
    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: std::marker::PhantomData,
    };
    let _ = visitor.visit_seq(seq);
}

