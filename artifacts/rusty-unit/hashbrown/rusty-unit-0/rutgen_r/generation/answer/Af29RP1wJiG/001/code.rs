// Answer 0

#[derive(Debug)]
struct MapEntry<'a, K, V> {
    key: &'a K,
    value: &'a V,
}

#[derive(Debug)]
struct MyTable<'a, K, V> {
    entries: Vec<MapEntry<'a, K, V>>,
}

impl<'a, K: PartialEq, V> MyTable<'a, K, V> {
    fn get<F>(&self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        for entry in &self.entries {
            if is_match(entry.key) {
                return Some((entry.key, entry.value));
            }
        }
        None
    }
}

#[derive(Debug)]
struct MyMap<'a, K, V> {
    table: MyTable<'a, K, V>,
}

impl<'a, K: PartialEq, V> MyMap<'a, K, V> {
    fn search<F>(&self, hash: u64, mut is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        self.table.get(hash, |(k, _)| is_match(k))
    }
}

#[test]
fn test_search_found() {
    let key1 = "key1";
    let value1 = "value1";
    let entries = vec![MapEntry { key: &key1, value: &value1 }];
    let table = MyTable { entries };
    let map = MyMap { table };

    let hash: u64 = 12345;
    let result = map.search(hash, |k| *k == "key1");

    assert_eq!(result, Some((&key1, &value1)));
}

#[test]
fn test_search_not_found() {
    let key1 = "key1";
    let value1 = "value1";
    let entries = vec![MapEntry { key: &key1, value: &value1 }];
    let table = MyTable { entries };
    let map = MyMap { table };

    let hash: u64 = 12345;
    let result = map.search(hash, |k| *k == "key2");

    assert_eq!(result, None);
}

