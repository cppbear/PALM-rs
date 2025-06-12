// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct MapSlice<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> MapSlice<K, V> {
    fn new(entries: Vec<Entry<K, V>>) -> Self {
        Self { entries }
    }

    fn into_entries(self: Box<Self>) -> Vec<Entry<K, V>> {
        self.entries
    }
}

struct IntoValues<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> IntoValues<K, V> {
    fn new(entries: Vec<Entry<K, V>>) -> Self {
        Self { entries }
    }
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.entries.pop() {
            Some(entry.value)
        } else {
            None
        }
    }
}

#[test]
fn test_into_values_empty() {
    let map_slice: Box<MapSlice<i32, i32>> = Box::new(MapSlice::new(Vec::new()));
    let values: IntoValues<i32, i32> = map_slice.into_values();
    let collected: Vec<i32> = values.collect();
    assert_eq!(collected, Vec::<i32>::new());
}

#[test]
fn test_into_values_single() {
    let entries = vec![Entry { key: 1, value: 10 }];
    let map_slice: Box<MapSlice<i32, i32>> = Box::new(MapSlice::new(entries));
    let values: IntoValues<i32, i32> = map_slice.into_values();
    let collected: Vec<i32> = values.collect();
    assert_eq!(collected, vec![10]);
}

#[test]
fn test_into_values_multiple() {
    let entries = vec![
        Entry { key: 1, value: 10 },
        Entry { key: 2, value: 20 },
        Entry { key: 3, value: 30 },
    ];
    let map_slice: Box<MapSlice<i32, i32>> = Box::new(MapSlice::new(entries));
    let values: IntoValues<i32, i32> = map_slice.into_values();
    let collected: Vec<i32> = values.collect();
    assert_eq!(collected, vec![30, 20, 10]);
}

