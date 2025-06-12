// Answer 0

#[test]
fn test_next_success() {
    struct TestDelegate {
        values: Vec<u8>,
        index: usize,
    }

    impl TestDelegate {
        fn new(values: Vec<u8>) -> Self {
            Self { values, index: 0 }
        }
    }

    impl Iterator for TestDelegate {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        delegate: TestDelegate,
    }

    impl TestStruct {
        fn new(delegate: TestDelegate) -> Self {
            Self { delegate }
        }

        fn next(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(self.delegate.next())
        }
    }

    let mut test_struct = TestStruct::new(TestDelegate::new(vec![1, 2, 3]));

    assert_eq!(test_struct.next().unwrap(), Some(1));
    assert_eq!(test_struct.next().unwrap(), Some(2));
    assert_eq!(test_struct.next().unwrap(), Some(3));
    assert_eq!(test_struct.next().unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_panic() {
    struct PanickingDelegate;

    impl Iterator for PanickingDelegate {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            panic!("This delegate panics on next call");
        }
    }

    struct PanickingStruct {
        delegate: PanickingDelegate,
    }

    impl PanickingStruct {
        fn new(delegate: PanickingDelegate) -> Self {
            Self { delegate }
        }

        fn next(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(self.delegate.next())
        }
    }

    let mut panicking_struct = PanickingStruct::new(PanickingDelegate);
    panicking_struct.next().unwrap();
}

