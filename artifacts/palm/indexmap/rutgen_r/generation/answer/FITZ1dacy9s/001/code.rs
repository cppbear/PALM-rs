// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct Slice<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> Slice<K, V> {
    fn from_slice(entries: &[Entry<K, V>]) -> Self {
        Self {
            entries: entries.to_vec(),
        }
    }

    pub fn split_first(&self) -> Option<(&K, Self)> {
        if let [first, rest @ ..] = &self.entries[..] {
            Some((&first.key, Self::from_slice(rest)))
        } else {
            None
        }
    }
}

#[test]
fn test_split_first_non_empty() {
    let entry1 = Entry { key: "key1", value: "value1" };
    let entry2 = Entry { key: "key2", value: "value2" };
    let slice = Slice {
        entries: vec![entry1, entry2],
    };

    let result = slice.split_first();
    assert!(result.is_some());

    if let Some((first_key, rest_slice)) = result {
        assert_eq!(first_key, &"key1");
        assert_eq!(rest_slice.entries.len(), 1);
        assert_eq!(rest_slice.entries[0].key, "key2");
    }
}

#[test]
fn test_split_first_empty() {
    let slice = Slice { entries: vec![] };
    
    let result = slice.split_first();
    assert!(result.is_none());
}

#[test]
fn test_split_first_single_entry() {
    let entry = Entry { key: "key1", value: "value1" };
    let slice = Slice {
        entries: vec![entry],
    };

    let result = slice.split_first();
    assert!(result.is_some());

    if let Some((first_key, rest_slice)) = result {
        assert_eq!(first_key, &"key1");
        assert!(rest_slice.entries.is_empty());
    }
}

