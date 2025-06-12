// Answer 0

#[derive(Default)]
struct MySet {
    map: std::collections::HashMap<i32, i32>,
}

impl MySet {
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[test]
fn test_is_empty_true_when_empty() {
    let set = MySet::default();
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_false_when_not_empty() {
    let mut set = MySet::default();
    set.map.insert(1, 1);
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_false_after_clearing() {
    let mut set = MySet::default();
    set.map.insert(1, 1);
    set.map.clear();
    assert!(set.is_empty());
}

