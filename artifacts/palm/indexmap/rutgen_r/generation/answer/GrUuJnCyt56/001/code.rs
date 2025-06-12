// Answer 0

#[derive(Default)]
struct TestSet {
    map: std::collections::HashMap<i32, bool>,
}

impl TestSet {
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[test]
fn test_clear_empty_set() {
    let mut set = TestSet::default();
    set.clear();
    assert!(set.map.is_empty());
}

#[test]
fn test_clear_non_empty_set() {
    let mut set = TestSet::default();
    set.map.insert(1, true);
    set.map.insert(2, true);
    set.clear();
    assert!(set.map.is_empty());
}

#[test]
fn test_clear_large_set() {
    let mut set = TestSet::default();
    for i in 0..1000 {
        set.map.insert(i, true);
    }
    set.clear();
    assert!(set.map.is_empty());
}

#[test]
fn test_clear_with_capacity() {
    let mut set = TestSet::default();
    for i in 0..10 {
        set.map.insert(i, true);
    }
    let capacity_before = set.map.capacity();
    set.clear();
    assert!(set.map.is_empty());
    assert_eq!(capacity_before, set.map.capacity());
}

