// Answer 0

#[test]
fn test_truncate_with_less_than_length() {
    struct TestStruct {
        core: Vec<(usize, usize)>,
    }

    impl TestStruct {
        fn truncate(&mut self, len: usize) {
            self.core.truncate(len);
        }
    }

    let mut map = TestStruct {
        core: vec![(1, 1), (2, 2), (3, 3)],
    };
    map.truncate(2);
    assert_eq!(map.core.len(), 2);
    assert_eq!(map.core, vec![(1, 1), (2, 2)]);
}

#[test]
fn test_truncate_with_equal_length() {
    struct TestStruct {
        core: Vec<(usize, usize)>,
    }

    impl TestStruct {
        fn truncate(&mut self, len: usize) {
            self.core.truncate(len);
        }
    }

    let mut map = TestStruct {
        core: vec![(1, 1), (2, 2), (3, 3)],
    };
    map.truncate(3);
    assert_eq!(map.core.len(), 3);
    assert_eq!(map.core, vec![(1, 1), (2, 2), (3, 3)]);
}

#[test]
fn test_truncate_with_more_than_length() {
    struct TestStruct {
        core: Vec<(usize, usize)>,
    }

    impl TestStruct {
        fn truncate(&mut self, len: usize) {
            self.core.truncate(len);
        }
    }

    let mut map = TestStruct {
        core: vec![(1, 1), (2, 2), (3, 3)],
    };
    map.truncate(5);
    assert_eq!(map.core.len(), 3);
    assert_eq!(map.core, vec![(1, 1), (2, 2), (3, 3)]);
}

#[test]
fn test_truncate_empty_map() {
    struct TestStruct {
        core: Vec<(usize, usize)>,
    }

    impl TestStruct {
        fn truncate(&mut self, len: usize) {
            self.core.truncate(len);
        }
    }

    let mut map = TestStruct { core: vec![] };
    map.truncate(0);
    assert_eq!(map.core.len(), 0);
    assert_eq!(map.core, vec![]);
}

