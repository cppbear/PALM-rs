// Answer 0

#[derive(Debug)]
struct TestKey(u64);
#[derive(Debug)]
struct TestValue(u64);
struct TestHashBuilder;

impl TestHashBuilder {
    fn build(&self, key: &TestKey) -> u64 {
        key.0
    }
}

struct TestMap {
    table: TestTable,
    hash_builder: TestHashBuilder,
}

struct TestTable {
    entries: Vec<(TestKey, TestValue)>,
}

impl TestTable {
    fn find<F>(&self, hash: u64, is_match: F) -> Option<&(TestKey, TestValue)>
    where
        F: FnMut(&(TestKey, TestValue)) -> bool,
    {
        self.entries.iter().find(is_match)
    }
}

struct RawEntryMut<'a, K, V, S, A> {
    map: RawEntryMap<'a, K, V, S, A>,
}

struct RawEntryMap<'a, K, V, S, A> {
    table: &'a mut TestTable,
    hash_builder: &'a S,
}

struct RawOccupiedEntryMut<'a, K, V, S> {
    elem: &'a (K, V),
    table: &'a mut TestTable,
    hash_builder: &'a S,
}

struct RawVacantEntryMut<'a, S> {
    table: &'a mut TestTable,
    hash_builder: &'a S,
}

impl<'a> RawEntryMut<'a, TestKey, TestValue, TestHashBuilder, ()> {
    fn search<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, TestKey, TestValue, TestHashBuilder, ()>
    where
        for<'b> F: FnMut(&'b TestKey) -> bool,
    {
        match self.map.table.find(hash, |(k, _)| is_match(k)) {
            Some(elem) => RawEntryMut {
                map: RawEntryMap {
                    table: self.map.table,
                    hash_builder: self.map.hash_builder,
                },
            },
            None => RawEntryMut {
                map: RawEntryMap {
                    table: self.map.table,
                    hash_builder: self.map.hash_builder,
                },
            },
        }
    }
}

#[test]
fn test_search_finds_existing_entry() {
    let key = TestKey(1);
    let value = TestValue(42);
    let mut table = TestTable {
        entries: vec![(key, value)],
    };
    let map = RawEntryMut {
        map: RawEntryMap {
            table: &mut table,
            hash_builder: &TestHashBuilder,
        },
    };
    let result = map.search(1, |k| k.0 == 1);
    match result.map {
        RawEntryMap { table, .. } => {
            assert_eq!(table.entries.len(), 1);
            assert_eq!(table.entries[0].0.0, 1);
            assert_eq!(table.entries[0].1.0, 42);
        }
    }
}

#[test]
fn test_search_returns_vacant_if_not_found() {
    let value = TestValue(42);
    let mut table = TestTable {
        entries: vec![(TestKey(1), value)],
    };
    let map = RawEntryMut {
        map: RawEntryMap {
            table: &mut table,
            hash_builder: &TestHashBuilder,
        },
    };
    let result = map.search(2, |k| k.0 == 2);
    match result.map {
        RawEntryMap { table, .. } => {
            assert_eq!(table.entries.len(), 1);
            assert_eq!(table.entries[0].0.0, 1);
            assert_eq!(table.entries[0].1.0, 42);
        }
    }
}

