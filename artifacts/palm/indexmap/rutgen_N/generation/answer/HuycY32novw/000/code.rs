// Answer 0

#[derive(Default)]
struct Set<T: Eq + std::hash::Hash> {
    map: std::collections::HashMap<T, ()>,
}

impl<T: Eq + std::hash::Hash> Set<T> {
    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, existing) = self.map.insert_full(value, ());
        (index, existing.is_none())
    }
}

// Test functions
#[test]
fn test_insert_full_new_value() {
    let mut set: Set<i32> = Set::default();
    let (index, inserted) = set.insert_full(42);
    assert_eq!(index, 0);
    assert!(inserted);
}

#[test]
fn test_insert_full_existing_value() {
    let mut set: Set<i32> = Set::default();
    let (index_first, _) = set.insert_full(42);
    let (index_second, inserted) = set.insert_full(42);
    assert_eq!(index_first, 0);
    assert_eq!(index_second, 0);
    assert!(!inserted);
}

#[test]
fn test_insert_full_multiple_values() {
    let mut set: Set<i32> = Set::default();
    let (index_first, _) = set.insert_full(10);
    let (index_second, _) = set.insert_full(20);
    let (index_third, inserted) = set.insert_full(10);
    
    assert_eq!(index_first, 0);
    assert_eq!(index_second, 1);
    assert_eq!(index_third, 0);
    assert!(!inserted);
}

