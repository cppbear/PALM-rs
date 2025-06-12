// Answer 0

#[derive(Debug, Hash, PartialEq, Eq)]
struct TestItem {
    value: i32,
}

#[derive(Default)]
struct TestSet {
    map: std::collections::HashMap<TestItem, usize>,
}

impl TestSet {
    pub fn get_index_of<Q>(&self, value: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Eq,
    {
        if let Some(val) = self.map.get(value) {
            Some(*val)
        } else {
            None
        }
    }

    pub fn insert(&mut self, item: TestItem, index: usize) {
        self.map.insert(item, index);
    }
}

#[test]
fn test_get_index_of_existing_item() {
    let mut set = TestSet::default();
    set.insert(TestItem { value: 1 }, 0);
    set.insert(TestItem { value: 2 }, 1);
    
    assert_eq!(set.get_index_of(&TestItem { value: 1 }), Some(0));
}

#[test]
fn test_get_index_of_non_existing_item() {
    let mut set = TestSet::default();
    set.insert(TestItem { value: 1 }, 0);
    
    assert_eq!(set.get_index_of(&TestItem { value: 2 }), None);
}

#[test]
fn test_get_index_of_empty_set() {
    let set = TestSet::default();
    
    assert_eq!(set.get_index_of(&TestItem { value: 1 }), None);
}

#[test]
fn test_get_index_of_item_with_different_type() {
    let mut set = TestSet::default();
    set.insert(TestItem { value: 1 }, 0);
    
    // This test should use an item that cannot be matched against TestItem
    let result: Option<usize> = set.get_index_of(&"wrong type");
    assert_eq!(result, None);
}

