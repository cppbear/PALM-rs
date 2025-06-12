// Answer 0

#[test]
fn test_next_with_some_value() {
    struct MockDelegate {
        values: Vec<Option<u8>>,
        index: usize,
    }

    impl MockDelegate {
        fn new(values: Vec<Option<u8>>) -> Self {
            Self { values, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>, String> {
            if self.index < self.values.len() {
                let result = self.values[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Err("No more values".to_string())
            }
        }
    }

    let mut delegate = MockDelegate::new(vec![Some(1), Some(2), None, Some(3)]);
    
    assert_eq!(delegate.next().unwrap(), Some(1));
    assert_eq!(delegate.next().unwrap(), Some(2));
    assert_eq!(delegate.next().unwrap(), None);
    assert_eq!(delegate.next(), Err("No more values".to_string()));
}

#[test]
fn test_next_with_no_value() {
    struct MockDelegate {
        values: Vec<Option<u8>>,
        index: usize,
    }

    impl MockDelegate {
        fn new(values: Vec<Option<u8>>) -> Self {
            Self { values, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>, String> {
            if self.index < self.values.len() {
                let result = self.values[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Err("No more values".to_string())
            }
        }
    }

    let mut delegate = MockDelegate::new(Vec::new());

    assert_eq!(delegate.next(), Err("No more values".to_string()));
}

#[test]
#[should_panic(expected = "No more values")]
fn test_next_panic_on_empty() {
    struct MockDelegate {
        values: Vec<Option<u8>>,
        index: usize,
    }

    impl MockDelegate {
        fn new(values: Vec<Option<u8>>) -> Self {
            Self { values, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>, String> {
            if self.index < self.values.len() {
                let result = self.values[self.index];
                self.index += 1;
                Ok(result)
            } else {
                panic!("No more values");
            }
        }
    }

    let mut delegate = MockDelegate::new(vec![Some(1), Some(2)]);
    let _ = delegate.next().unwrap();
    let _ = delegate.next().unwrap();
    let _ = delegate.next().unwrap(); // This should panic
}

