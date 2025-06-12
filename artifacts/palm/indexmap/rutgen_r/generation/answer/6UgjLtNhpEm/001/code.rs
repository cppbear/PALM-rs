// Answer 0


#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

impl<K, V> Bucket<K, V> {
    fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
}

struct MyMap<K, V> {
    entries: Vec<Bucket<K, V>>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap { entries: Vec::new() }
    }
    
    fn as_entries(&self) -> &Vec<Bucket<K, V>> {
        &self.entries
    }
    
    pub fn first(&self) -> Option<(&K, &V)> {
        self.as_entries().first().map(Bucket::refs)
    }
}

#[test]
fn test_first_with_empty_map() {
    let my_map: MyMap<i32, &str> = MyMap::new();
    assert_eq!(my_map.first(), None);
}

#[test]
fn test_first_with_one_entry() {
    let mut my_map = MyMap::new();
    my_map.entries.push(Bucket { key: 1, value: "one" });
    assert_eq!(my_map.first(), Some((&1, &"one")));
}

#[test]
fn test_first_with_multiple_entries() {
    let mut my_map = MyMap::new();
    my_map.entries.push(Bucket { key: 1, value: "one" });
    my_map.entries.push(Bucket { key: 2, value: "two" });
    my_map.entries.push(Bucket { key: 3, value: "three" });
    assert_eq!(my_map.first(), Some((&1, &"one")));
}

#[test]
fn test_first_with_various_types() {
    let mut my_map = MyMap::new();
    my_map.entries.push(Bucket { key: 'a', value: 100 });
    assert_eq!(my_map.first(), Some((&'a', &100)));
}

#[test]
#[should_panic]
fn test_first_with_potential_panic() {
    let my_map: MyMap<String, String> = MyMap::new();
    my_map.first().unwrap(); // Expected to panic since it is empty
}


