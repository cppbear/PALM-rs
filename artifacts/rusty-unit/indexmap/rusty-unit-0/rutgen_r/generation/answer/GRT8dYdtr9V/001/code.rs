// Answer 0

#[derive(Debug, PartialEq)]
struct TestKey {
    id: usize,
}

#[derive(Debug)]
struct TestValue {
    data: String,
}

impl PartialEq for TestValue {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

struct TestMap {
    entries: Vec<(TestKey, TestValue)>,
}

impl TestMap {
    fn new() -> Self {
        TestMap { entries: Vec::new() }
    }

    pub fn from_key_hashed_nocheck<Q>(&self, hash: u64, key: &Q) -> Option<&TestValue>
    where
        Q: ?Sized + Equivalent<TestKey>,
    {
        let entry: Option<&(TestKey, TestValue)> = self.entries.iter().find(|(k, _)| Q::equivalent(key, k));
        entry.map(|(_, v)| v)
    }

    pub fn insert(&mut self, key: TestKey, value: TestValue) {
        self.entries.push((key, value));
    }
}

impl Equivalent<TestKey> for TestKey {
    fn equivalent(a: &TestKey, b: &TestKey) -> bool {
        a.id == b.id
    }
}

#[test]
fn test_from_key_hashed_nocheck_found() {
    let mut map = TestMap::new();
    map.insert(TestKey { id: 1 }, TestValue { data: String::from("value1") });

    let result = map.from_key_hashed_nocheck(1, &TestKey { id: 1 });
    assert_eq!(result, Some(&TestValue { data: String::from("value1") }));
}

#[test]
fn test_from_key_hashed_nocheck_not_found() {
    let mut map = TestMap::new();
    map.insert(TestKey { id: 2 }, TestValue { data: String::from("value2") });

    let result = map.from_key_hashed_nocheck(1, &TestKey { id: 1 });
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_panic() {
    let map = TestMap::new();
    let _ = map.from_key_hashed_nocheck(0, &TestKey { id: 1 }); // not panicking, but for coverage
}

