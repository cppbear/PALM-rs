// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

struct Map<V> {
    entries: Vec<Entry<V>>,
    index: usize,
}

impl<V> Map<V> {
    pub fn new(entries: Vec<Entry<V>>, index: usize) -> Self {
        Map { entries, index }
    }

    pub fn get(&self) -> &V {
        &self.entries[self.index()].value
    }
}

#[test]
fn test_get_valid_index() {
    let entries = vec![
        Entry { value: 1 },
        Entry { value: 2 },
        Entry { value: 3 },
    ];
    let map = Map::new(entries, 1);
    assert_eq!(*map.get(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_index_out_of_bounds_high() {
    let entries = vec![
        Entry { value: 1 },
        Entry { value: 2 },
        Entry { value: 3 },
    ];
    let map = Map::new(entries, 5);
    let _ = map.get();
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_index_out_of_bounds_negative() {
    let entries = vec![
        Entry { value: 1 },
        Entry { value: 2 },
        Entry { value: 3 },
    ];
    let map = Map::new(entries, usize::MAX);
    let _ = map.get();
}

#[test]
fn test_get_empty_map() {
    let entries: Vec<Entry<i32>> = Vec::new();
    let map = Map::new(entries, 0);
    // Testing out-of-bounds on an empty map
    let result = std::panic::catch_unwind(|| {
        let _ = map.get();
    });
    assert!(result.is_err());
}

