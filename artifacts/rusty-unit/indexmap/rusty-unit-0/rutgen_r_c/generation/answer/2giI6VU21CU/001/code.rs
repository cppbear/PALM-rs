// Answer 0

#[test]
fn test_erase_index_not_found() {
    struct TestHashTable {
        entries: Vec<usize>,
    }

    impl TestHashTable {
        fn find_entry(&self, _hash: u64, _predicate: impl Fn(&&usize) -> bool) -> Result<TestEntry, ()> {
            Err(())
        }
    }

    struct TestEntry;

    impl TestEntry {
        fn remove(self) {}
    }

    let mut table = TestHashTable { entries: Vec::new() };
    let hash = HashValue(1);
    let index = 0;

    // This test should not panic as we are simulating that the entry was not found
    let result = std::panic::catch_unwind(|| {
        erase_index(&mut table, hash, index);
    });

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_erase_index_should_panic() {
    struct TestHashTable {
        entries: Vec<usize>,
    }

    impl TestHashTable {
        fn find_entry(&self, _hash: u64, _predicate: impl Fn(&&usize) -> bool) -> Result<TestEntry, ()> {
            Err(())
        }
    }

    struct TestEntry;

    impl TestEntry {
        fn remove(self) {}
    }

    let mut table = TestHashTable { entries: Vec::new() };
    let hash = HashValue(1);
    let index = 0;

    // This test should panic as we expect the entry not to be found and debug assertions are enabled
    let result = std::panic::catch_unwind(|| {
        erase_index(&mut table, hash, index);
    });

    assert!(result.is_err());
}

