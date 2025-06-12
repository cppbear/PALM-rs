// Answer 0

#[derive(Debug)]
struct MyMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn as_entries_mut(&mut self) -> &mut Vec<(K, V)> {
        &mut self.data
    }
}

struct IterMut2<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    index: usize,
}

impl<'a, K, V> IterMut2<'a, K, V> {
    fn new(entries: &'a mut Vec<(K, V)>) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'a, K, V> Iterator for IterMut2<'a, K, V> {
    type Item = &'a mut (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let item = &mut self.entries[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[test]
fn test_iter_mut2() {
    let mut my_map: MyMap<i32, String> = MyMap::new();
    my_map.data.push((1, "one".to_string()));
    my_map.data.push((2, "two".to_string()));
    
    let iter = my_map.iter_mut2();
    let mut keys: Vec<i32> = vec![];
    let mut values: Vec<String> = vec![];

    for (key, value) in iter {
        keys.push(*key);
        values.push(value.clone());
    }

    assert_eq!(keys, vec![1, 2]);
    assert_eq!(values, vec!["one", "two"]);
}

