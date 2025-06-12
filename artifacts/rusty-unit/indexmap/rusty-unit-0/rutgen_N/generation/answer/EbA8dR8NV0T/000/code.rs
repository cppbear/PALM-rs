// Answer 0

#[test]
fn test_sorted_unstable_by() {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<i32, &'static str>,
    }

    impl MyMap {
        fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(3, "three");
            map.insert(1, "one");
            map.insert(2, "two");
            Self { map }
        }

        fn into_entries(self) -> Vec<Entry<i32, &'static str>> {
            self.map.into_iter()
                .map(|(k, v)| Entry { key: k, value: v })
                .collect()
        }
    }

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IntoIter<K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
    }

    impl<K, V> IntoIter<K, V> {
        fn new(entries: Vec<Entry<K, V>>) -> Self {
            Self { entries, index: 0 }
        }
    }

    // Using a custom comparator to sort
    let my_map = MyMap::new();
    let sorted_entries: Vec<Entry<i32, &'static str>> = my_map.into_entries();
    sorted_entries.sort_unstable_by(|a, b| {
        if a.key < b.key {
            Ordering::Less
        } else if a.key > b.key {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    let mut iter = IntoIter::new(sorted_entries);
    assert_eq!(iter.entries[0].key, 1);
    assert_eq!(iter.entries[0].value, "one");
    assert_eq!(iter.entries[1].key, 2);
    assert_eq!(iter.entries[1].value, "two");
    assert_eq!(iter.entries[2].key, 3);
    assert_eq!(iter.entries[2].value, "three");
}

