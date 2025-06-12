// Answer 0

#[test]
fn test_size_hint_none() {
    struct IteratorStruct {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for IteratorStruct {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: IteratorStruct,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self {
                iter: IteratorStruct { data, index: 0 },
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![1, 2, 3];
    let test_struct = TestStruct::new(test_data);

    assert_eq!(test_struct.size_hint(), None);
}

#[test]
fn test_size_hint_none_boundaries() {
    struct IteratorStruct {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for IteratorStruct {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let lower = self.data.len();
            let upper = Some(self.data.len());
            (lower, upper)
        }
    }

    struct TestStruct {
        iter: IteratorStruct,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self {
                iter: IteratorStruct { data, index: 0 },
            }
        }

        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let test_data = vec![]; // An empty iterator, lower = 0, upper = Some(0)
    let test_struct = TestStruct::new(test_data);

    assert_eq!(test_struct.size_hint(), None);
}

