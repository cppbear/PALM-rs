// Answer 0

#[test]
fn test_visit_seq_with_valid_values() {
    struct ValidSeqAccess {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for ValidSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value as T))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.values.len() - self.index)
        }
    }

    let seq_access = ValidSeqAccess { values: vec![98, 101, 115, 116], index: 0 };
    let c_string_result: Result<CString, _> = seq_access.visit_seq(seq_access);

    assert!(c_string_result.is_ok());
}

#[test]
fn test_visit_seq_with_empty_sequence() {
    struct EmptySeqAccess;

    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let seq_access = EmptySeqAccess;
    let c_string_result: Result<CString, _> = seq_access.visit_seq(seq_access);

    assert!(c_string_result.is_err());
}

#[test]
fn test_visit_seq_with_error_condition() {
    struct ErrorSeqAccess {
        index: usize,
    }

    impl<'de> SeqAccess<'de> for ErrorSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.index == 0 {
                self.index += 1;
                Err(serde::de::Error::custom("Forced error"))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(1) // Indicating one element to read
        }
    }

    let mut seq_access = ErrorSeqAccess { index: 0 };
    let c_string_result: Result<CString, _> = seq_access.visit_seq(seq_access);

    assert!(c_string_result.is_err());
}

