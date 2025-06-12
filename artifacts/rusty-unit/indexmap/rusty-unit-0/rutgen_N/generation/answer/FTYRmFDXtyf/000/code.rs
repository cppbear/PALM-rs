// Answer 0

#[derive(Debug)]
struct TestSet<T> {
    map: std::collections::HashMap<T, ()>,
}

impl<T: std::hash::Hash + Eq> TestSet<T> {
    pub fn new() -> Self {
        TestSet {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }
}

#[test]
fn test_insert_new_value() {
    let mut set = TestSet::new();
    assert!(set.insert(1));
    assert_eq!(set.map.len(), 1);
}

#[test]
fn test_insert_existing_value() {
    let mut set = TestSet::new();
    set.insert(1);
    assert!(!set.insert(1));
    assert_eq!(set.map.len(), 1);
}

#[test]
fn test_insert_different_values() {
    let mut set = TestSet::new();
    assert!(set.insert(1));
    assert!(set.insert(2));
    assert_eq!(set.map.len(), 2);
}

#[test]
fn test_insert_with_string() {
    let mut set = TestSet::new();
    assert!(set.insert("hello".to_string()));
    assert!(!set.insert("hello".to_string()));
    assert_eq!(set.map.len(), 1);
}

#[test]
fn test_insert_boundary_conditions() {
    let mut set = TestSet::new();
    assert!(set.insert(0));
    assert!(!set.insert(0));
    assert_eq!(set.map.len(), 1);
    
    assert!(set.insert(usize::MAX));
    assert!(!set.insert(usize::MAX));
    assert_eq!(set.map.len(), 2);
}

