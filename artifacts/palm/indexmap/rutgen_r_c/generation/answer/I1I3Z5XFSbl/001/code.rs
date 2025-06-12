// Answer 0

#[test]
fn test_binary_search_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            // Simplified return for test purposes
            // In a real implementation, this should return a valid Slice containing self.data
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![1, 3, 5, 7, 9]);
    assert_eq!(sorted_set.binary_search(&5), Ok(2));
}

#[test]
fn test_binary_search_not_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![1, 3, 5, 7, 9]);
    assert_eq!(sorted_set.binary_search(&6), Err(3));
}

#[test]
fn test_binary_search_empty() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![]);
    assert_eq!(sorted_set.binary_search(&5), Err(0));
}

#[test]
fn test_binary_search_first_element() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(sorted_set.binary_search(&1), Ok(0));
}

#[test]
fn test_binary_search_last_element() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(sorted_set.binary_search(&5), Ok(4));
}

#[test]
fn test_binary_search_duplicate_elements() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            TestSet { data }
        }

        fn as_slice(&self) -> &Slice<i32> {
            Box::into_raw(Box::new(Slice { entries: self.data.clone().into_boxed_slice().try_into().unwrap() }))
        }
    }

    let sorted_set = TestSet::new(vec![1, 2, 2, 3, 4]);
    assert_eq!(sorted_set.binary_search(&2), Ok(1)); // Might return index of the first occurrence
}

