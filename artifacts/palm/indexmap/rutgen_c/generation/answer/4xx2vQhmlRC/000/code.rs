// Answer 0

#[test]
fn test_partition_point_empty() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }
    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
        
        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
        
        fn partition_point<F>(&self, pred: F) -> usize
        where
            F: FnMut(&i32, &i32) -> bool,
        {
            self.as_slice().iter().position(|(k, v)| !pred(k, v)).unwrap_or(self.entries.len())
        }
    }
    
    let map = TestMap::new();
    let predicate = |k: &i32, _: &i32| *k < 0;
    assert_eq!(map.partition_point(predicate), 0);
}

#[test]
fn test_partition_point_one_element_true() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }
    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10)] }
        }
        
        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
        
        fn partition_point<F>(&self, pred: F) -> usize
        where
            F: FnMut(&i32, &i32) -> bool,
        {
            self.as_slice().iter().position(|(k, v)| !pred(k, v)).unwrap_or(self.entries.len())
        }
    }
    
    let map = TestMap::new();
    let predicate = |k: &i32, _: &i32| *k < 2;
    assert_eq!(map.partition_point(predicate), 1);
}

#[test]
fn test_partition_point_one_element_false() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }
    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10)] }
        }
        
        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
        
        fn partition_point<F>(&self, pred: F) -> usize
        where
            F: FnMut(&i32, &i32) -> bool,
        {
            self.as_slice().iter().position(|(k, v)| !pred(k, v)).unwrap_or(self.entries.len())
        }
    }
    
    let map = TestMap::new();
    let predicate = |k: &i32, _: &i32| *k < 0;
    assert_eq!(map.partition_point(predicate), 0);
}

#[test]
fn test_partition_point_multiple_elements() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }
    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10), (2, 20), (3, 30)] }
        }
        
        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
        
        fn partition_point<F>(&self, pred: F) -> usize
        where
            F: FnMut(&i32, &i32) -> bool,
        {
            self.as_slice().iter().position(|(k, v)| !pred(k, v)).unwrap_or(self.entries.len())
        }
    }
    
    let map = TestMap::new();
    let predicate = |k: &i32, _: &i32| *k < 2;
    assert_eq!(map.partition_point(predicate), 1);
}

#[test]
fn test_partition_point_all_false() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }
    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![(1, 10), (2, 20), (3, 30)] }
        }
        
        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
        
        fn partition_point<F>(&self, pred: F) -> usize
        where
            F: FnMut(&i32, &i32) -> bool,
        {
            self.as_slice().iter().position(|(k, v)| !pred(k, v)).unwrap_or(self.entries.len())
        }
    }
    
    let map = TestMap::new();
    let predicate = |_: &i32, _: &i32| false;
    assert_eq!(map.partition_point(predicate), 0);
}

