// Answer 0

#[derive(Debug)]
struct HashValue {
    value: usize,
}

impl HashValue {
    fn get(&self) -> usize {
        self.value
    }
}

trait Equivalent<K> {
    fn equivalent(&self, other: &K) -> bool;
}

#[derive(Debug)]
struct TestKey {
    id: usize,
}

impl Equivalent<TestKey> for TestKey {
    fn equivalent(&self, other: &TestKey) -> bool {
        self.id == other.id
    }
}

#[derive(Default)]
struct TestIndex {
    entries: Vec<(HashValue, TestKey)>,
}

impl TestIndex {
    fn find_entry<Q>(&self, hash: usize, eq: Q) -> Result<(&usize, &TestKey), ()>
    where
        Q: Fn(&TestKey) -> bool,
    {
        for (index, entry) in self.entries.iter().enumerate() {
            if hash == entry.0.get() && eq(&entry.1) {
                return Ok((&index, &entry.1));
            }
        }
        Err(())
    }
}

struct TestMap {
    indices: TestIndex,
    entries: Vec<(HashValue, TestKey, String)>, // (hash, key, value)
}

impl TestMap {
    pub(crate) fn swap_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, TestKey, String)>
    where
        Q: ?Sized + Equivalent<TestKey>,
    {
        let eq = |k: &TestKey| key.equivalent(k);
        match self.indices.find_entry(hash.get(), eq) {
            Ok((index, _)) => {
                let (key, value) = self.entries.swap_remove(*index);
                Some((*index, key, value))
            }
            Err(_) => None,
        }
    }
}

#[test]
fn test_swap_remove_full_success() {
    let mut map = TestMap {
        indices: TestIndex {
            entries: vec![(HashValue { value: 1 }, TestKey { id: 1 })],
        },
        entries: vec![(HashValue { value: 1 }, TestKey { id: 1 }, "value1".into())],
    };

    let result = map.swap_remove_full(HashValue { value: 1 }, &TestKey { id: 1 });
    assert_eq!(result, Some((0, TestKey { id: 1 }, "value1".into())));
}

#[test]
fn test_swap_remove_full_not_found() {
    let mut map = TestMap {
        indices: TestIndex {
            entries: vec![(HashValue { value: 1 }, TestKey { id: 1 })],
        },
        entries: vec![(HashValue { value: 1 }, TestKey { id: 1 }, "value1".into())],
    };

    let result = map.swap_remove_full(HashValue { value: 2 }, &TestKey { id: 2 });
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_multiple_entries() {
    let mut map = TestMap {
        indices: TestIndex {
            entries: vec![
                (HashValue { value: 1 }, TestKey { id: 1 }),
                (HashValue { value: 2 }, TestKey { id: 2 }),
            ],
        },
        entries: vec![
            (HashValue { value: 1 }, TestKey { id: 1 }, "value1".into()),
            (HashValue { value: 2 }, TestKey { id: 2 }, "value2".into()),
        ],
    };

    let result = map.swap_remove_full(HashValue { value: 2 }, &TestKey { id: 2 });
    assert_eq!(result, Some((1, TestKey { id: 2 }, "value2".into())));
    assert_eq!(map.entries.len(), 1); // Ensure the entry was removed
}

