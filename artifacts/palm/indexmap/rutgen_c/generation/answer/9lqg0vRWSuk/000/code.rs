// Answer 0

#[test]
fn test_binary_search_by_key_existing_value() {
    struct TestSet {
        items: Vec<i32>,
    }

    impl TestSet {
        fn new(items: Vec<i32>) -> Self {
            TestSet { items }
        }
    
        fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.items.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.items[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.items.len() && f(&self.items[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let test_set = TestSet::new(vec![1, 3, 5, 7, 9]);
    
    assert_eq!(test_set.binary_search_by_key(&5, |&x| x), Ok(2));
}

#[test]
fn test_binary_search_by_key_non_existing_value() {
    struct TestSet {
        items: Vec<i32>,
    }

    impl TestSet {
        fn new(items: Vec<i32>) -> Self {
            TestSet { items }
        }
    
        fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.items.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.items[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.items.len() && f(&self.items[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let test_set = TestSet::new(vec![1, 3, 5, 7, 9]);
    
    assert_eq!(test_set.binary_search_by_key(&6, |&x| x), Err(3));
}

#[test]
fn test_binary_search_by_key_edge_case() {
    struct TestSet {
        items: Vec<i32>,
    }

    impl TestSet {
        fn new(items: Vec<i32>) -> Self {
            TestSet { items }
        }
    
        fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.items.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.items[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.items.len() && f(&self.items[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let test_set = TestSet::new(vec![1, 3, 5, 7, 9]);
    
    assert_eq!(test_set.binary_search_by_key(&1, |&x| x), Ok(0));
    assert_eq!(test_set.binary_search_by_key(&9, |&x| x), Ok(4));
}

