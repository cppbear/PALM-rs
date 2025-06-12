// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct TestIndexMap {
        core: Vec<(usize, usize)>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { core: vec![(1, 1), (2, 2), (3, 3)] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let len = self.core.len();
            assert!(from < len);
            assert!(to < len);
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = TestIndexMap::new();
    map.move_index(0, 2);
    assert_eq!(map.core, vec![(2, 2), (3, 3), (1, 1)]); // Element 1 moved from index 0 to 2
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_move_index_from_out_of_bounds() {
    struct TestIndexMap {
        core: Vec<(usize, usize)>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { core: vec![(1, 1), (2, 2), (3, 3)] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let len = self.core.len();
            assert!(from < len);
            assert!(to < len);
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = TestIndexMap::new();
    map.move_index(3, 1); // Panic expected: from index 3 is out of bounds
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_move_index_to_out_of_bounds() {
    struct TestIndexMap {
        core: Vec<(usize, usize)>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { core: vec![(1, 1), (2, 2), (3, 3)] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let len = self.core.len();
            assert!(from < len);
            assert!(to < len);
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = TestIndexMap::new();
    map.move_index(1, 3); // Panic expected: to index 3 is out of bounds
}

#[test]
fn test_move_index_reverse_shift() {
    struct TestIndexMap {
        core: Vec<(usize, usize)>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { core: vec![(1, 1), (2, 2), (3, 3)] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let len = self.core.len();
            assert!(from < len);
            assert!(to < len);
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = TestIndexMap::new();
    map.move_index(2, 0);
    assert_eq!(map.core, vec![(3, 3), (1, 1), (2, 2)]); // Element 2 moved from index 2 to 0
}

