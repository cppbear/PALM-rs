// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

struct Map<V> {
    entries: Vec<Entry<V>>,
}

impl<V> Map<V> {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn push(&mut self, value: V) {
        self.entries.push(Entry { value });
    }

    fn get(&self, index: usize) -> &V {
        &self.entries[index].value
    }
}

#[test]
fn test_get_value() {
    let mut map: Map<i32> = Map::new();
    map.push(42);
    let value = map.get(0);
    assert_eq!(*value, 42);
}

#[test]
fn test_get_empty_map_should_panic() {
    let map: Map<i32> = Map::new();
    let result = std::panic::catch_unwind(|| {
        map.get(0);
    });
    assert!(result.is_err());
}

