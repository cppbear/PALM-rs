// Answer 0

#[test]
fn test_binary_search_existing_value() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            let mut set = TestSet { data };
            set.data.sort(); // Ensure the data is sorted for binary search
            set
        }

        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.data.binary_search(x)
        }
    }

    let set = TestSet::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(set.binary_search(&3), Ok(2));
}

#[test]
fn test_binary_search_missing_value() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            let mut set = TestSet { data };
            set.data.sort(); // Ensure the data is sorted for binary search
            set
        }

        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.data.binary_search(x)
        }
    }

    let set = TestSet::new(vec![1, 2, 4, 5]);
    assert_eq!(set.binary_search(&3), Err(2)); // Should return the position where 3 can be inserted
}

#[test]
fn test_binary_search_edge_cases() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(data: Vec<i32>) -> Self {
            let mut set = TestSet { data };
            set.data.sort(); // Ensure the data is sorted for binary search
            set
        }

        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.data.binary_search(x)
        }
    }

    let set_empty = TestSet::new(vec![]);
    assert_eq!(set_empty.binary_search(&1), Err(0)); // Inserting into empty set

    let set_one_element = TestSet::new(vec![1]);
    assert_eq!(set_one_element.binary_search(&1), Ok(0)); // Existing element
    assert_eq!(set_one_element.binary_search(&0), Err(0)); // Missing, goes before
    assert_eq!(set_one_element.binary_search(&2), Err(1)); // Missing, goes after
}

