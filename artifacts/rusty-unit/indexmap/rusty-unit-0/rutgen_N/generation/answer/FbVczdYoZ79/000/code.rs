// Answer 0

#[test]
fn test_binary_search_by_found() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap {
        entries: Vec<Entry<i32, &'static str>>,
    }

    impl SortedMap {
        fn new(entries: Vec<Entry<i32, &'static str>>) -> Self {
            SortedMap { entries }
        }

        pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &'static str) -> std::cmp::Ordering,
        {
            self.entries.binary_search_by(move |a| f(&a.key, &a.value))
        }
    }

    let entries = vec![
        Entry { key: 1, value: "one" },
        Entry { key: 2, value: "two" },
        Entry { key: 3, value: "three" },
    ];
    let map = SortedMap::new(entries);

    let result = map.binary_search_by(|&key, _| key.cmp(&2));
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found_insert_position() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap {
        entries: Vec<Entry<i32, &'static str>>,
    }

    impl SortedMap {
        fn new(entries: Vec<Entry<i32, &'static str>>) -> Self {
            SortedMap { entries }
        }

        pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &'static str) -> std::cmp::Ordering,
        {
            self.entries.binary_search_by(move |a| f(&a.key, &a.value))
        }
    }

    let entries = vec![
        Entry { key: 1, value: "one" },
        Entry { key: 2, value: "two" },
        Entry { key: 3, value: "three" },
    ];
    let map = SortedMap::new(entries);

    let result = map.binary_search_by(|&key, _| key.cmp(&4));
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_empty() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct SortedMap {
        entries: Vec<Entry<i32, &'static str>>,
    }

    impl SortedMap {
        fn new(entries: Vec<Entry<i32, &'static str>>) -> Self {
            SortedMap { entries }
        }

        pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a i32, &'a &'static str) -> std::cmp::Ordering,
        {
            self.entries.binary_search_by(move |a| f(&a.key, &a.value))
        }
    }

    let map = SortedMap::new(vec![]);

    let result = map.binary_search_by(|&key, _| key.cmp(&1));
    assert_eq!(result, Err(0));
}

