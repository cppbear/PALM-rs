// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.data.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.data[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.data.len() && f(&self.data[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search_by_key(&5, |&x| x);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.data.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.data[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.data.len() && f(&self.data[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search_by_key(&6, |&x| x);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_lower_bound() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.data.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.data[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.data.len() && f(&self.data[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search_by_key(&1, |&x| x);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_key_upper_bound() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.data.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if f(&self.data[mid]) < *b {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            if low < self.data.len() && f(&self.data[low]) == *b {
                Ok(low)
            } else {
                Err(low)
            }
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search_by_key(&10, |&x| x);
    assert_eq!(result, Err(5));
}

