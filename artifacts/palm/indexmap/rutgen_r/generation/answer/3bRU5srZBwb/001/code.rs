// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

impl<K, V> Bucket<K, V> {
    fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }
}

#[derive(Default)]
struct MyMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> MyMap<K, V> {
    fn as_entries_mut(&mut self) -> &mut Vec<Bucket<K, V>> {
        &mut self.buckets
    }

    fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(Bucket::muts)
    }
}

#[test]
fn test_get_index_mut2_valid_index() {
    let mut map = MyMap::default();
    map.buckets.push(Bucket { key: 1, value: "value1" });
    map.buckets.push(Bucket { key: 2, value: "value2" });

    let (key, value) = map.get_index_mut2(1).expect("Should not panic");
    *key = 3; 
    *value = "value3";
    
    assert_eq!(map.buckets[1].key, 3);
    assert_eq!(map.buckets[1].value, "value3");
}

#[test]
fn test_get_index_mut2_out_of_bounds() {
    let mut map = MyMap::default();
    map.buckets.push(Bucket { key: 1, value: "value1" });

    let result = map.get_index_mut2(2);
    assert!(result.is_none());
}

