// Answer 0

#[test]
fn test_visit_seq_valid() {
    struct TestSeq {
        elements: Vec<(Idx, Idx)>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let elem = self.elements[self.index];
                self.index += 1;
                Ok(Some(elem))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        elements: vec![(1, 2)], // Providing valid inputs
        index: 0,
    };

    let result: Result<(Idx, Idx), _> = visit_seq(seq);
    assert_eq!(result.unwrap(), (1, 2));
}

#[test]
#[should_panic]
fn test_visit_seq_no_start() {
    struct TestSeq {
        elements: Vec<(Idx, Idx)>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            Ok(None) // No elements provided
        }
    }

    let seq = TestSeq {
        elements: vec![],
        index: 0,
    };

    let result: Result<(Idx, Idx), _> = visit_seq(seq); // Should trigger error
}

#[test]
#[should_panic]
fn test_visit_seq_no_end() {
    struct TestSeq {
        elements: Vec<(Idx, Idx)>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.index == 0 {
                self.index += 1;
                Ok(Some((1, 2))) // Providing only start
            } else {
                Ok(None) // No second element
            }
        }
    }

    let seq = TestSeq {
        elements: vec![(1, 2)],
        index: 0,
    };

    let result: Result<(Idx, Idx), _> = visit_seq(seq); // Should trigger error
}

