// Answer 0

#[test]
fn test_visit_seq_empty() {
    struct MockSeqAccess {
        elements: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value = serde_json::from_value(serde_json::json!(self.elements[self.index]))?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.elements.len())
        }
    }

    let seq = MockSeqAccess { elements: vec![], index: 0 };
    let visitor = CStringVisitor;
    
    let result: Result<CString, _> = visitor.visit_seq(seq);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "");
}

#[test]
fn test_visit_seq_with_values() {
    struct MockSeqAccess {
        elements: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value = serde_json::from_value(serde_json::json!(self.elements[self.index]))?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.elements.len())
        }
    }

    let seq = MockSeqAccess { elements: vec![97, 98, 99], index: 0 };
    let visitor = CStringVisitor;

    let result: Result<CString, _> = visitor.visit_seq(seq);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_str().unwrap(), "abc");
}

#[test]
fn test_visit_seq_invalid_utf8() {
    struct MockSeqAccess {
        elements: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.elements.len() {
                let value = serde_json::from_value(serde_json::json!(self.elements[self.index]))?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.elements.len())
        }
    }

    let seq = MockSeqAccess { elements: vec![255, 255, 255], index: 0 };
    let visitor = CStringVisitor;

    let result: Result<CString, _> = visitor.visit_seq(seq);
    
    assert!(result.is_err());
}

