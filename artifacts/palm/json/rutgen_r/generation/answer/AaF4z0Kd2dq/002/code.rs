// Answer 0

#[test]
fn test_next_index_at_length() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn next(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            })
        }
    }

    let mut test_instance = TestStruct {
        slice: vec![1, 2, 3],
        index: 3, // self.index == self.slice.len()
    };

    let result = test_instance.next();
    assert_eq!(result, Ok(None));
}

