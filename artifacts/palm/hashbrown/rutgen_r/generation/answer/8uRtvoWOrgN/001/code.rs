// Answer 0

#[derive(Debug)]
struct TestEntry {
    value: i32,
}

struct TestTable {
    data: Vec<(u64, TestEntry)>,
}

impl TestTable {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, key: u64, entry: TestEntry) {
        self.data.push((key, entry));
    }

    pub fn get_many_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        mut eq: impl FnMut(usize, &TestEntry) -> bool,
    ) -> [Option<&'_ mut TestEntry>; N] {
        // Simulating the original function logic for testing
        let mut results = [None; N];

        for (i, hash) in hashes.iter().enumerate() {
            if let Some(pos) = self.data.iter().position(|(key, _)| key == hash) {
                if results[..i].iter().any(|r| r.is_some() && eq(i, &self.data[pos].1)) {
                    panic!("duplicate keys found");
                }
                results[i] = Some(&mut self.data[pos].1);
            }
        }

        results
    }
}

#[test]
fn test_get_many_mut_with_distinct_keys() {
    let mut table = TestTable::new();
    table.insert(1, TestEntry { value: 10 });
    table.insert(2, TestEntry { value: 20 });
    table.insert(3, TestEntry { value: 30 });

    let hashes = [1, 2, 3];
    let result = table.get_many_mut(hashes, |i, entry| {
        entry.value == (10 * (i as i32 + 1)) // Simulate equality based on entry value
    });

    assert!(result[0].is_some() && result[0].as_ref().unwrap().value == 10);
    assert!(result[1].is_some() && result[1].as_ref().unwrap().value == 20);
    assert!(result[2].is_some() && result[2].as_ref().unwrap().value == 30);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_with_duplicate_keys() {
    let mut table = TestTable::new();
    table.insert(1, TestEntry { value: 10 });
    table.insert(2, TestEntry { value: 20 });
    table.insert(3, TestEntry { value: 30 });

    let hashes = [1, 2, 2]; // Duplicate key '2' should trigger a panic.
    let _result = table.get_many_mut(hashes, |i, entry| {
        entry.value == (10 * (i as i32 + 1)) // Simulate equality based on entry value
    });
}

#[test]
fn test_get_many_mut_with_non_existing_keys() {
    let mut table = TestTable::new();
    table.insert(1, TestEntry { value: 10 });
    table.insert(3, TestEntry { value: 30 });

    let hashes = [2, 3]; // Hash '2' does not exist
    let result = table.get_many_mut(hashes, |i, entry| {
        entry.value == (10 * (i as i32 + 1)) // Simulate equality based on entry value
    });

    assert!(result[0].is_none());
    assert!(result[1].is_some() && result[1].as_ref().unwrap().value == 30);
}

