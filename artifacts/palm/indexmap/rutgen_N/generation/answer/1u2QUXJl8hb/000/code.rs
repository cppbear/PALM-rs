// Answer 0

#[derive(Debug)]
struct MyMap<K, V> {
    core: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap { core: Vec::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        self.core.push((key, value));
    }

    fn reverse(&mut self) {
        self.core.reverse()
    }
}

#[test]
fn test_reverse_empty_map() {
    let mut map: MyMap<i32, i32> = MyMap::new();
    map.reverse();
    assert!(map.core.is_empty());
}

#[test]
fn test_reverse_single_element() {
    let mut map = MyMap::new();
    map.insert(1, 2);
    map.reverse();
    assert_eq!(map.core, vec![(1, 2)]);
}

#[test]
fn test_reverse_multiple_elements() {
    let mut map = MyMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);
    map.reverse();
    assert_eq!(map.core, vec![(5, 6), (3, 4), (1, 2)]);
}

#[test]
fn test_reverse_two_elements() {
    let mut map = MyMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.reverse();
    assert_eq!(map.core, vec![(3, 4), (1, 2)]);
}

