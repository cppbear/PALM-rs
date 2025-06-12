// Answer 0

#[derive(Default)]
struct MySet {
    map: std::collections::HashMap<i32, ()>,
}

impl MySet {
    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn insert(&mut self, value: i32) {
        self.map.insert(value, ());
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

#[test]
fn test_clear_empty_set() {
    let mut set = MySet::default();
    set.clear();
    assert_eq!(set.len(), 0);
}

#[test]
fn test_clear_non_empty_set() {
    let mut set = MySet::default();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert_eq!(set.len(), 3);
    set.clear();
    assert_eq!(set.len(), 0);
}

#[test]
fn test_clear_set_with_capacity() {
    let mut set = MySet::default();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.clear();
    set.insert(4);
    assert_eq!(set.len(), 1);
    assert_eq!(set.map.len(), 1); // Ensure capacity is preserved
}

