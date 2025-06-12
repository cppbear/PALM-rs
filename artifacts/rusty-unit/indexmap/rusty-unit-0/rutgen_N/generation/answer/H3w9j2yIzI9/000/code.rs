// Answer 0

#[test]
fn test_insert_sorted_unique_value() {
    struct SimpleSet {
        map: std::collections::BTreeMap<i32, ()>,
    }
    
    impl SimpleSet {
        fn new() -> Self {
            SimpleSet {
                map: std::collections::BTreeMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = SimpleSet::new();
    let (index, inserted) = set.insert_sorted(5);
    assert_eq!(index, 0);
    assert!(inserted);

    let (index, inserted) = set.insert_sorted(10);
    assert_eq!(index, 1);
    assert!(inserted);

    let (index, inserted) = set.insert_sorted(5);
    assert_eq!(index, 0);
    assert!(!inserted);
}

#[test]
fn test_insert_sorted_duplicate_value() {
    struct SimpleSet {
        map: std::collections::BTreeMap<i32, ()>,
    }
    
    impl SimpleSet {
        fn new() -> Self {
            SimpleSet {
                map: std::collections::BTreeMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = SimpleSet::new();
    let (index, inserted) = set.insert_sorted(5);
    assert!(inserted);
    assert_eq!(index, 0);

    let (index, inserted) = set.insert_sorted(5);
    assert!(!inserted);
    assert_eq!(index, 0);
}

#[test]
fn test_insert_sorted_boundary_conditions() {
    struct SimpleSet {
        map: std::collections::BTreeMap<i32, ()>,
    }
    
    impl SimpleSet {
        fn new() -> Self {
            SimpleSet {
                map: std::collections::BTreeMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = SimpleSet::new();

    assert_eq!(set.insert_sorted(i32::MIN), (0, true));
    assert_eq!(set.insert_sorted(0), (1, true));
    assert_eq!(set.insert_sorted(i32::MAX), (2, true));
    assert_eq!(set.insert_sorted(0), (1, false));
}

