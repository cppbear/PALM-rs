// Answer 0

#[test]
fn test_visit_seq_with_valid_sequence() {
    struct MockSeqAccess {
        next_element: Option<i32>,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(self.next_element.take().map(|v| v as T))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };
    let mut seq = MockSeqAccess { next_element: Some(42) };
    let result = visitor.visit_seq(&mut seq).unwrap();
    assert_eq!(result, 42);
}

#[test]
#[should_panic]
fn test_visit_seq_with_empty_sequence() {
    struct MockSeqAccess {
        next_element: Option<i32>,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(self.next_element.take().map(|v| v as T))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };
    let mut seq = MockSeqAccess { next_element: None };
    let _ = visitor.visit_seq(&mut seq).unwrap();
}

#[test]
fn test_visit_seq_with_primitive_type() {
    struct MockSeqAccess {
        next_elements: Vec<Option<i32>>,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = crate::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(self.next_elements.pop().and_then(|v| v.map(|v| v as T)))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };
    
    let mut seq = MockSeqAccess { next_elements: vec![Some(27)] };
    let result = visitor.visit_seq(&mut seq).unwrap();
    assert_eq!(result, 27);
}

