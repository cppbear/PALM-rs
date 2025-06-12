// Answer 0

#[test]
fn test_binary_search_existing_value() {
    struct TestSet {
        data: Vec<i32>,
    }
    
    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }
        
        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search(&5);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_non_existing_value() {
    struct TestSet {
        data: Vec<i32>,
    }
    
    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }
        
        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    let result = set.binary_search(&4);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }
    
    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }
        
        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let set = TestSet { data: vec![] };
    let result = set.binary_search(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_boundary_values() {
    struct TestSet {
        data: Vec<i32>,
    }
    
    impl TestSet {
        fn as_slice(&self) -> &[i32] {
            &self.data
        }
        
        fn binary_search(&self, x: &i32) -> Result<usize, usize> {
            self.as_slice().binary_search(x)
        }
    }

    let set = TestSet { data: vec![1, 3, 5, 7, 9] };
    assert_eq!(set.binary_search(&1), Ok(0));  // Lower boundary
    assert_eq!(set.binary_search(&9), Ok(4));  // Upper boundary
    assert_eq!(set.binary_search(&0), Err(0));  // Below lower boundary
    assert_eq!(set.binary_search(&10), Err(5)); // Above upper boundary
}

