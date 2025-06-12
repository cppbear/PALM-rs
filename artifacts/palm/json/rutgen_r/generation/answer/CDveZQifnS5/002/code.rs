// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct SizeHintIterator {
        data: Vec<i32>,
    }

    impl SizeHintIterator {
        fn new(data: Vec<i32>) -> Self {
            SizeHintIterator { data }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len();
            (len, Some(len))
        }
    }

    struct MyStruct {
        iter: SizeHintIterator,
    }

    impl MyStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let data = vec![1, 2, 3]; // A case where lower == upper; len is 3
    let my_struct = MyStruct {
        iter: SizeHintIterator::new(data),
    };

    assert_eq!(my_struct.size_hint(), Some(3));
}

#[test]
fn test_size_hint_empty() {
    struct SizeHintIterator {
        data: Vec<i32>,
    }

    impl SizeHintIterator {
        fn new(data: Vec<i32>) -> Self {
            SizeHintIterator { data }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len();
            (len, Some(len))
        }
    }

    struct MyStruct {
        iter: SizeHintIterator,
    }

    impl MyStruct {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let data: Vec<i32> = Vec::new(); // A case where lower == upper and is 0
    let my_struct = MyStruct {
        iter: SizeHintIterator::new(data),
    };

    assert_eq!(my_struct.size_hint(), Some(0));
}

