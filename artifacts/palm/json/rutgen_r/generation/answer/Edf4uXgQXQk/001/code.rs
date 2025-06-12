// Answer 0

#[test]
fn test_size_hint_none() {
    struct MyIter {
        data: Vec<i32>,
    }

    impl Iterator for MyIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    struct MyCollection {
        iter: MyIter,
    }

    impl MyCollection {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let collection = MyCollection {
        iter: MyIter { data: vec![1, 2, 3] }, // This setup will not satisfy lower == upper
    };

    assert_eq!(collection.size_hint(), None);
}

#[test]
fn test_size_hint_none_with_zero_elements() {
    struct MyIter {
        data: Vec<i32>,
    }

    impl Iterator for MyIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.data.pop()
        }
    }

    struct MyCollection {
        iter: MyIter,
    }

    impl MyCollection {
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let collection = MyCollection {
        iter: MyIter { data: vec![] }, // Lower and upper will be (0, 0), but doesn't trigger lower == upper
    };

    assert_eq!(collection.size_hint(), None);
}

