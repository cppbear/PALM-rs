// Answer 0

#[derive(Debug)]
struct MyMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap { data: Vec::new() }
    }

    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        if index >= self.data.len() {
            return None;
        }
        let entry = self.data.remove(index);
        Some(entry)
    }

    fn insert(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }
}

#[test]
fn test_shift_remove_index_valid() {
    let mut map = MyMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let removed = map.shift_remove_index(1);
    assert_eq!(removed, Some(("b", 2)));
    assert_eq!(map.data.len(), 2);
    assert_eq!(map.data[0], ("a", 1));
    assert_eq!(map.data[1], ("c", 3));
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    let mut map = MyMap::new();
    map.insert("a", 1);
    
    let removed = map.shift_remove_index(1);
    assert_eq!(removed, None);
    assert_eq!(map.data.len(), 1);
}

#[test]
fn test_shift_remove_index_empty() {
    let mut map: MyMap<&str, i32> = MyMap::new();
    
    let removed = map.shift_remove_index(0);
    assert_eq!(removed, None);
    assert!(map.data.is_empty());
}

