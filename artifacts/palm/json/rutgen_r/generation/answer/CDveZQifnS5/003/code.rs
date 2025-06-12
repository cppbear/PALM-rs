// Answer 0

#[test]
fn test_size_hint_lower_not_equal_upper() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: TestIter,
    }

    impl TestStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let iter = TestIter {
        data: vec![1, 2, 3], // 3 elements
        index: 0,
    };
    
    let test_struct = TestStruct { iter };

    assert_eq!(test_struct.size_hint(), None); // lower = 3, upper = 3 
}

