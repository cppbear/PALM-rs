// Answer 0

#[test]
fn test_pop_non_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }
        
        fn push(&mut self, key: K, value: V) {
            let index = self.entries.len();
            self.entries.push((key, value));
            self.indices.push(index);
        }

        pub(crate) fn pop(&mut self) -> Option<(K, V)> {
            if let Some(entry) = self.entries.pop() {
                let last = self.entries.len();
                erase_index(&mut self.indices, last);
                Some((entry.0, entry.1))
            } else {
                None
            }
        }
    }

    fn erase_index<K>(indices: &mut Vec<usize>, last: usize) {
        if let Some(pos) = indices.iter().position(|&x| x == last) {
            indices.remove(pos);
        }
    }
    
    let mut map = TestMap::new();
    map.push("key1", "value1");
    map.push("key2", "value2");

    let result = map.pop();
    assert_eq!(result, Some(("key2", "value2")));
    assert_eq!(map.entries.len(), 1);
}

#[test]
fn test_pop_empty() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        pub(crate) fn pop(&mut self) -> Option<(K, V)> {
            if let Some(entry) = self.entries.pop() {
                let last = self.entries.len();
                erase_index(&mut self.indices, last);
                Some((entry.0, entry.1))
            } else {
                None
            }
        }
    }

    fn erase_index<K>(indices: &mut Vec<usize>, last: usize) {
        if let Some(pos) = indices.iter().position(|&x| x == last) {
            indices.remove(pos);
        }
    }

    let mut map = TestMap::new();
    let result = map.pop();
    assert_eq!(result, None);
}

