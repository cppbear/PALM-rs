// Answer 0


struct ValuesMut<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    index: usize,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    fn new(entries: &'a mut Vec<(K, V)>) -> Self {
        ValuesMut { entries, index: 0 }
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let value = &mut self.entries[self.index].1;
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

struct MapSlice<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> MapSlice<K, V> {
    fn new() -> Self {
        MapSlice { entries: Vec::new() }
    }

    fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut::new(&mut self.entries)
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push((key, value));
    }
}

#[test]
fn test_values_mut_empty() {
    let mut map_slice: MapSlice<i32, String> = MapSlice::new();
    let mut iter = map_slice.values_mut();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_values_mut_single_element() {
    let mut map_slice = MapSlice::new();
    map_slice.insert(1, "value1".to_string());
    let mut iter = map_slice.values_mut();
    assert_eq!(iter.next(), Some(&mut "value1".to_string()));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_values_mut_multiple_elements() {
    let mut map_slice = MapSlice::new();
    map_slice.insert(1, "value1".to_string());
    map_slice.insert(2, "value2".to_string());
    
    let mut iter = map_slice.values_mut();
    assert_eq!(iter.next(), Some(&mut "value1".to_string()));
    assert_eq!(iter.next(), Some(&mut "value2".to_string()));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_values_mut_panic_condition() {
    let mut map_slice = MapSlice::new();
    map_slice.insert(1, "value1".to_string());
    map_slice.insert(2, "value2".to_string());
    
    {
        let mut iter = map_slice.values_mut();
        if let Some(value) = iter.next() {
            *value = "modified".to_string();
        }
    }
    
    let mut iter = map_slice.values_mut();
    assert_eq!(iter.next(), Some(&mut "modified".to_string()));
    assert_eq!(iter.next(), Some(&mut "value2".to_string()));
}


