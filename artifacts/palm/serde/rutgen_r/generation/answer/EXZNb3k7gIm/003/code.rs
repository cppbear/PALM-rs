// Answer 0

fn test_visit_seq_ok() -> Result<(), Box<dyn std::error::Error>> {
    use serde::de::{SeqAccess, DeserializeSeed};
    use serde::de::IgnoredAny;

    struct TestSeqAccess {
        elements: Vec<Result<IgnoredAny, String>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = String;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<IgnoredAny>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.elements.len() {
                let val = self.elements[self.index].clone();
                self.index += 1;
                return Ok(Some(val?));
            }
            Ok(None)
        }
    }

    let seq = TestSeqAccess {
        elements: vec![Ok(IgnoredAny), Ok(IgnoredAny), Ok(IgnoredAny)],
        index: 0,
    };

    let result: Result<IgnoredAny, String> = visit_seq(seq);
    assert!(result.is_ok());
    Ok(())
}

fn test_visit_seq_err() -> Result<(), Box<dyn std::error::Error>> {
    use serde::de::{SeqAccess, DeserializeSeed};
    use serde::de::IgnoredAny;

    struct TestSeqAccess {
        elements: Vec<Result<IgnoredAny, String>>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = String;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<IgnoredAny>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.elements.len() {
                let val = self.elements[self.index].clone();
                self.index += 1;
                return Ok(Some(val?));
            }
            Ok(None)
        }
    }

    let seq = TestSeqAccess {
        elements: vec![Ok(IgnoredAny), Err("Error occurred".to_string())],
        index: 0,
    };

    let result: Result<IgnoredAny, String> = visit_seq(seq);
    assert!(result.is_err());
    Ok(())
}

