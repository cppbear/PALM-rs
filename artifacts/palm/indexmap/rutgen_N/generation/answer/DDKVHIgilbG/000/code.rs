// Answer 0

#[derive(Debug)]
struct MySet<T> {
    map: IndexMap<usize, T>,
}

impl<T> MySet<T> {
    fn new() -> Self {
        MySet {
            map: IndexMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {
        self.map.swap_remove_index(index).map(|(x, ())| x)
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.map.insert(key, value);
    }
}

// Test cases for MySet
#[test]
fn test_swap_remove_index_valid() {
    let mut set = MySet::new();
    set.insert(0, "a");
    set.insert(1, "b");
    set.insert(2, "c");

    assert_eq!(set.swap_remove_index(1), Some("b"));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_swap_remove_index_last_element() {
    let mut set = MySet::new();
    set.insert(0, "x");
    set.insert(1, "y");

    assert_eq!(set.swap_remove_index(1), Some("y"));
    assert_eq!(set.len(), 1);
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    let mut set = MySet::new();
    set.insert(0, "first");

    assert_eq!(set.swap_remove_index(1), None);
    assert_eq!(set.len(), 1);
}

#[test]
fn test_swap_remove_index_empty_set() {
    let mut set: MySet<&str> = MySet::new();

    assert_eq!(set.swap_remove_index(0), None);
    assert_eq!(set.len(), 0);
}

